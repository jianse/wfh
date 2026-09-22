use std::path::Path;

use tantivy::Index;
use tantivy::IndexReader;
use tantivy::IndexWriter;
use tantivy::TantivyDocument;
use tantivy::Term;
use tantivy::collector::Count;
use tantivy::collector::TopDocs;
use tantivy::directory::MmapDirectory;
use tantivy::doc;
use tantivy::query::QueryParser;
use tantivy::schema::Field;
use tantivy::schema::IndexRecordOption;
use tantivy::schema::STORED;
use tantivy::schema::Schema;
use tantivy::schema::TEXT;
use tantivy::schema::TextFieldIndexing;
use tantivy::schema::TextOptions;
use tantivy::schema::Value;
use tantivy::tokenizer::LowerCaser;
use tantivy::tokenizer::RemoveLongFilter;
use tantivy::tokenizer::Stemmer;
use tantivy::tokenizer::TextAnalyzer;

pub use tantivy::TantivyError as Error;

const TOKENIZER_NAME: &str = "jieba";
const UNIQUE_NAME_FIELD: &str = "unique_name";
const TEXT_FIELD: &str = "text";

/// A tantivy index over the language key/value table.
///
/// The `text` (translation) field is indexed with a jieba tokenizer so Chinese
/// content can be searched. The `unique_name` field is stored, so a search hit
/// can be mapped back to the in-memory language map.
pub struct LangIndex {
    index: Index,
    reader: IndexReader,
    unique_name_field: Field,
    text_field: Field,
}

impl std::fmt::Debug for LangIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LangIndex")
            .field("unique_name_field", &self.unique_name_field)
            .field("text_field", &self.text_field)
            .finish_non_exhaustive()
    }
}

impl LangIndex {
    /// Open an existing index or create a new empty one in `dir`.
    pub fn open_or_create<P: AsRef<Path>>(dir: P) -> Result<Self, Error> {
        let dir = dir.as_ref();
        std::fs::create_dir_all(dir)?;

        let index = Index::open_or_create(MmapDirectory::open(dir)?, Self::schema())?;
        Self::register_tokenizer(&index);

        let schema = index.schema();
        let unique_name_field = schema.get_field(UNIQUE_NAME_FIELD)?;
        let text_field = schema.get_field(TEXT_FIELD)?;
        let reader = index.reader()?;

        Ok(Self {
            index,
            reader,
            unique_name_field,
            text_field,
        })
    }

    /// Open/create `dir` and (re)build the index from `items`.
    pub fn build<'a, P, I>(dir: P, items: I) -> Result<Self, Error>
    where
        P: AsRef<Path>,
        I: IntoIterator<Item = (&'a str, &'a str)>,
    {
        let lang_index = Self::open_robust(dir)?;
        lang_index.rebuild(items)?;
        Ok(lang_index)
    }

    /// Open/create `dir` and only rebuild when the number of indexed documents
    /// does not match the number of `items`.
    ///
    /// Single entry edits are applied incrementally, so a restart with unchanged
    /// data does not reindex everything.
    pub fn ensure<'a, P, I>(dir: P, items: I) -> Result<Self, Error>
    where
        P: AsRef<Path>,
        I: IntoIterator<Item = (&'a str, &'a str)> + Clone,
    {
        let lang_index = Self::open_robust(dir)?;
        let expected = items.clone().into_iter().count();
        if lang_index.reader.searcher().num_docs() as usize != expected {
            lang_index.rebuild(items)?;
        }
        Ok(lang_index)
    }

    /// Open an existing index, recreating the directory if it is incompatible
    /// (e.g. an old schema).
    fn open_robust<P: AsRef<Path>>(dir: P) -> Result<Self, Error> {
        let dir = dir.as_ref();
        match Self::open_or_create(dir) {
            Ok(index) => Ok(index),
            Err(err) => {
                tracing::warn!("failed to open language search index ({err}), recreating");
                let _ = std::fs::remove_dir_all(dir);
                Self::open_or_create(dir)
            }
        }
    }

    fn schema() -> Schema {
        let mut builder = Schema::builder();

        builder.add_text_field(UNIQUE_NAME_FIELD, TEXT | STORED);
        builder.add_text_field(
            TEXT_FIELD,
            TextOptions::default().set_indexing_options(
                TextFieldIndexing::default()
                    .set_tokenizer(TOKENIZER_NAME)
                    .set_index_option(IndexRecordOption::WithFreqsAndPositions),
            ),
        );

        builder.build()
    }

    fn register_tokenizer(index: &Index) {
        let tokenizer = tantivy_jieba::JiebaTokenizer::new();
        let analyzer = TextAnalyzer::builder(tokenizer)
            .filter(RemoveLongFilter::limit(40))
            .filter(LowerCaser)
            .filter(Stemmer::default())
            .build();
        index.tokenizers().register(TOKENIZER_NAME, analyzer);
    }

    /// Drop all documents and index `items` from scratch.
    pub fn rebuild<'a, I>(&self, items: I) -> Result<(), Error>
    where
        I: IntoIterator<Item = (&'a str, &'a str)>,
    {
        let mut writer: IndexWriter = self.index.writer(50_000_000)?;
        writer.delete_all_documents()?;
        for (unique_name, text) in items {
            writer.add_document(doc!(
                self.unique_name_field => unique_name,
                self.text_field => text
            ))?;
        }
        writer.commit()?;
        self.reader.reload()?;
        Ok(())
    }

    /// Insert or replace a single entry.
    pub fn upsert(&self, unique_name: &str, text: &str) -> Result<(), Error> {
        let mut writer: IndexWriter = self.index.writer(15_000_000)?;
        writer.delete_term(Term::from_field_text(self.unique_name_field, unique_name));
        writer.add_document(doc!(
            self.unique_name_field => unique_name,
            self.text_field => text
        ))?;
        writer.commit()?;
        self.reader.reload()?;
        Ok(())
    }

    /// Remove a single entry.
    pub fn remove(&self, unique_name: &str) -> Result<(), Error> {
        let mut writer: IndexWriter = self.index.writer(15_000_000)?;
        writer.delete_term(Term::from_field_text(self.unique_name_field, unique_name));
        writer.commit()?;
        self.reader.reload()?;
        Ok(())
    }

    /// Search `query` and return `(unique_names, total_matches)`.
    ///
    /// `unique_names` are ordered by relevance and paginated by `offset`/`limit`.
    pub fn search(
        &self,
        query: &str,
        offset: usize,
        limit: usize,
    ) -> Result<(Vec<String>, usize), Error> {
        let limit = limit.max(1);
        let end = offset.saturating_add(limit);

        let searcher = self.reader.searcher();
        let parser =
            QueryParser::for_index(&self.index, vec![self.text_field, self.unique_name_field]);
        let query = parser.parse_query(query)?;

        let (top_docs, total) = searcher.search(
            &query,
            &(TopDocs::for_doc_range(offset..end).order_by_score(), Count),
        )?;

        let mut names = Vec::with_capacity(top_docs.len());
        for (_score, doc_address) in top_docs {
            let doc: TantivyDocument = searcher.doc(doc_address)?;
            if let Some(name) = doc
                .get_first(self.unique_name_field)
                .and_then(|value| value.as_str())
            {
                names.push(name.to_owned());
            }
        }
        Ok((names, total))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lang_index_round_trip() -> Result<(), Error> {
        let dir = std::env::temp_dir().join(format!("wf-lang-search-test-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);

        let index = LangIndex::build(
            &dir,
            [("k1", "orokin reactor"), ("k2", "forma blueprint")],
        )?;

        let (names, total) = index.search("reactor", 0, 10)?;
        assert_eq!(total, 1);
        assert_eq!(names, vec!["k1".to_owned()]);

        index.upsert("k1", "dark star")?;
        let (names, _) = index.search("dark", 0, 10)?;
        assert_eq!(names, vec!["k1".to_owned()]);
        let (_, total) = index.search("reactor", 0, 10)?;
        assert_eq!(total, 0);

        index.remove("k1")?;
        let (_, total) = index.search("dark", 0, 10)?;
        assert_eq!(total, 0);

        let _ = std::fs::remove_dir_all(&dir);
        Ok(())
    }
}
