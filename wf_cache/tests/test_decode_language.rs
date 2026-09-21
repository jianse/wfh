use get_size2::GetSize;
use hex::ToHex;
use liblzma::bufread::XzDecoder;
use reqwest::Client;
use reqwest::header::ETAG;
use serde::Deserialize;
use serde::Serialize;
use serde::de::value;
use std::collections::HashMap;
use std::io::Cursor;
use std::io::Read;
use std::path::Path;
use thiserror::Error;
use wf_cache::CachePair;
use wf_cache::decode_languages;

#[derive(Debug, Serialize, Deserialize, Default)]
struct CacheConfig {
    index_hash: String,
    h_misc_zh_toc_md5: String,
    h_misc_zh_cache_md5: String,
}

const ROOT_DIR: &str = "../cache/client";

fn init_debug_logging() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
}

#[tokio::test]
async fn test_get_index() -> eyre::Result<()> {
    init_debug_logging();

    let client = reqwest::Client::new();

    let index_url = "https://content-zhpt.wf.qq.com/origin/01234567/index.txt.lzma";
    let body = client.get(index_url).send().await?.bytes().await?;
    let mut dec = XzDecoder::new_multi_decoder(body.as_ref());

    let mut content = String::new();
    dec.read_to_string(&mut content)?;

    println!("{}", content);
    Ok(())
}

#[derive(Debug)]
struct ClientIndexItem<'a> {
    client_path: &'a str,
    hash: &'a str,
    size: usize,
}

impl<'a> ClientIndexItem<'a> {
    fn download_path(&self) -> String {
        format!("{}.{}.lzma", self.client_path, self.hash)
    }
}

#[derive(Debug, Error)]
enum ParseClientIndexItemError {
    #[error("Invalid format")]
    InvalidFormat,

    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),
}

impl<'a> TryFrom<&'a str> for ClientIndexItem<'a> {
    type Error = ParseClientIndexItemError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let (path_and_hash, size) = value
            .split_once(',')
            .ok_or(ParseClientIndexItemError::InvalidFormat)?;
        let size = size.parse::<usize>()?;
        let (client_path, rem) = path_and_hash
            .split_at_checked(path_and_hash.len() - 5 - 32 - 1)
            .ok_or(ParseClientIndexItemError::InvalidFormat)?;
        let hash = &rem[1..33];

        Ok(ClientIndexItem {
            client_path,
            hash,
            size,
        })
    }
}

#[derive(Debug)]
struct ClientIndex<'a>(HashMap<&'a str, ClientIndexItem<'a>>);

impl<'a> TryFrom<&'a str> for ClientIndex<'a> {
    type Error = ParseClientIndexItemError;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let mut res = Self(HashMap::new());
        for line in s.lines() {
            let item: ClientIndexItem<'_> = line.try_into()?;
            res.0.insert(item.client_path, item);
        }
        Ok(res)
    }
}

#[test]
fn test_parse_client_index() -> eyre::Result<()> {
    let str = std::fs::read_to_string("cache/client/index.txt")?;
    let mf = ClientIndex::try_from(str.as_str())?;
    dbg!(mf);
    Ok(())
}

#[tokio::test]
async fn test_decode_language() -> eyre::Result<()> {
    tracing_subscriber::fmt::init();

    let client = reqwest::Client::new();
    std::fs::create_dir_all(ROOT_DIR)?;
    // let config =
    let cp = Path::new(ROOT_DIR).join("cache_config.json");
    let mut config = if cp.exists() {
        let s = std::fs::read_to_string(&cp)?;
        serde_json::from_str(&s)?
    } else {
        CacheConfig::default()
    };

    let nonce = rand::random::<u32>();

    let index_url = format!(
        "https://content-zhpt.wf.qq.com/origin/{:08X}/index.txt.lzma",
        nonce
    );

    let resp = client.get(index_url).send().await?;

    let etag = resp
        .headers()
        .get(ETAG)
        .and_then(|h| h.to_str().ok())
        .unwrap_or_default()
        .to_string();

    let toc_file = Path::new(ROOT_DIR).join("H.Misc_zh.toc");
    let cache_file = Path::new(ROOT_DIR).join("H.Misc_zh.cache");

    if etag == config.index_hash {
        println!("cache hit")
    } else {
        config.index_hash = etag;

        let body = resp
            // .error_for_status()?
            .bytes()
            .await?;

        println!("{:?}", body.slice(..16).encode_hex::<String>());

        let mut dec = XzDecoder::new_multi_decoder(body.as_ref());

        let mut manifest = String::new();

        dec.read_to_string(&mut manifest)?;

        std::fs::write(Path::new(ROOT_DIR).join("index.txt"), &manifest)?;

        let index = ClientIndex::try_from(manifest.as_str())?;

        let cache_name = index.0.get("/Cache.Windows/H.Misc_zh.cache").unwrap();

        let mut cache_file_content =
            download_file_decompress(&client, &cache_name.download_path()).await?;

        let mut file = std::fs::File::create(&cache_file)?;
        std::io::copy(&mut cache_file_content, &mut file)?;

        config.h_misc_zh_cache_md5 = cache_name.hash.to_string();

        dbg!(index.0.keys());

        let toc_name = index.0.get("/Cache.Windows/H.Misc_zh.toc").unwrap();

        let mut toc_file_content =
            download_file_decompress(&client, &toc_name.download_path()).await?;

        let mut file = std::fs::File::create(&toc_file)?;
        std::io::copy(&mut toc_file_content, &mut file)?;

        config.h_misc_zh_toc_md5 = toc_name.hash.to_string();

        let s = serde_json::to_string_pretty(&config)?;

        std::fs::write(cp, s)?;

        // Ok(())
    }

    let mut cache_pair = CachePair::new(toc_file, cache_file);

    cache_pair.print_files();

    let langs = cache_pair.get_data_and_decompress("Languages.bin");

    let mut lang = decode_languages(&mut Cursor::new(langs))?;
    lang.sort_keys();

    std::fs::write(
        Path::new(ROOT_DIR).join("lang_zh.json"),
        serde_json::to_vec_pretty(&lang)?,
    )?;

    Ok(())
}

async fn download_file_decompress(client: &Client, file_name: &str) -> eyre::Result<Box<dyn Read>> {
    let body = client
        .get(format!("https://content-zhpt.wf.qq.com{}", file_name))
        .send()
        .await?
        .bytes()
        .await?;
    let dec = XzDecoder::new_multi_decoder(Cursor::new(body));
    Ok(Box::new(dec))
    // todo!("not implemented")
}

fn md5_file(path: &Path) -> eyre::Result<String> {
    let mut file = std::fs::File::open(path)?;
    let mut hasher = md5::Context::new();
    let mut buf = [0; 8 * 1024];
    while let Ok(n) = file.read(&mut buf)
        && n > 0
    {
        hasher.consume(&buf[..n]);
    }
    Ok(hasher.finalize().encode_hex_upper::<String>())
}

#[test]
fn test_format() {
    println!("{:08X}", 42);
}

#[derive(Debug, Default, GetSize)]
struct Leaf {
    value: Option<String>,
    branchs: HashMap<String, Leaf>,
}

#[derive(Debug, GetSize)]
struct Tree {
    root: Leaf,
}

impl Tree {
    fn new() -> Self {
        Self {
            root: Leaf::default(),
        }
    }

    fn insert(&mut self, path: &str, value: String) {
        let parts = path.split('/').filter(|e| !e.is_empty());

        let mut leaf = &mut self.root;
        for part in parts {
            leaf = leaf
                .branchs
                .entry(part.to_string())
                .or_insert(Leaf::default());
        }
        leaf.value = Some(value);
    }

    fn find(&self, path: &str) -> Option<&String> {
        let parts = path.split('/').filter(|e| !e.is_empty());
        let mut cur = &self.root;
        for part in parts {
            if let Some(leaf) = cur.branchs.get(part) {
                cur = leaf;
            } else {
                return None;
            }
        }
        return cur.value.as_ref();
    }
}
#[test]
fn test_tree() {
    let mut map = HashMap::new();
    map.insert("/a/b/c".to_string(), "hello".to_string());
    map.insert("/a/b/d".to_string(), "world".to_string());
    dbg!(map.get_size());

    map.shrink_to_fit();

    dbg!(map.get_size());

    let mut tree = Tree::new();
    tree.insert("/a/b/c", "hello".to_string());
    tree.insert("/a/b/d", "world".to_string());
    dbg!(&tree);

    dbg!(tree.get_size());
    dbg!(tree.root.get_size());
    dbg!(tree.root.value.get_size());
    dbg!(tree.root.branchs.get_size());

    assert_eq!(tree.find("/a/b/c"), Some(&"hello".to_string()));
    assert_eq!(tree.find("/a/b/d"), Some(&"world".to_string()));
}

#[test]
fn test_lang_size() {
    let p = Path::new(ROOT_DIR).join("lang_zh.json");
    let s = std::fs::read_to_string(p).unwrap();
    let mut lang = serde_json::from_str::<HashMap<String, String>>(&s).unwrap();
    dbg!(lang.get_size());
    lang.shrink_to_fit();
    dbg!(lang.get_size());

    // let mut lang2 = Tree::new();
    // for (k, v) in lang {
    //     lang2.insert(&k, v);
    // }

    // dbg!(lang2.get_size());
}
