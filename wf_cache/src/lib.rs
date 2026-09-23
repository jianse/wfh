use std::fs;
use std::io::BufReader;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::path::Path;
use std::path::PathBuf;

use crate::dir::DirTree;
use crate::dir::FileNode;
mod compress;
pub mod dir;
mod error;
mod fetch;
mod languages;

pub use error::CacheErr;
pub use fetch::fetch_languages;
pub use languages::decode as decode_languages;

#[derive(Debug)]
pub struct CachePair {
    // toc_path: PathBuf,
    cache_path: PathBuf,
    dir_tree: DirTree,
    // size: usize,
    has_read_toc: bool,
    cache_reader: std::io::BufReader<fs::File>,
}

impl CachePair {
    pub fn new<P1: AsRef<Path>, P2: AsRef<Path>>(toc_path: P1, cache_path: P2) -> Self {
        let mut ret = Self::new_uninit(toc_path, cache_path);
        ret.read_toc();
        ret
    }

    pub fn new_uninit<P1: AsRef<Path>, P2: AsRef<Path>>(toc_path: P1, cache_path: P2) -> Self {
        let file = fs::File::open(&cache_path).unwrap();
        let cache_reader = std::io::BufReader::new(file);
        Self {
            // toc_path: toc_path.as_ref().to_path_buf(),
            cache_path: cache_path.as_ref().to_path_buf(),
            dir_tree: DirTree::new(toc_path.as_ref().to_path_buf()),
            has_read_toc: false,
            cache_reader,
        }
    }

    pub fn read_toc(&mut self) {
        if self.has_read_toc {
            return;
        }
        self.dir_tree.read_toc_file();
        self.has_read_toc = true;
    }

    pub fn get_data_and_decompress(&mut self, file_name: &str) -> Vec<u8> {
        let entry = self.dir_tree.get_file_entry(file_name).unwrap();
        let borrowed_entry = entry.read().unwrap();
        if borrowed_entry.compressed_size == borrowed_entry.length {
            return self.get_data(&borrowed_entry);
        }

        compress::get_data_and_decompress_post(&borrowed_entry, &mut self.cache_reader)
    }

    pub fn get_data_by_name(&mut self, file_name: &str) -> Vec<u8> {
        let entry = self.dir_tree.get_file_entry(file_name).unwrap();
        let borrowed_entry = entry.read().unwrap();
        self.get_data(&borrowed_entry)
    }

    fn get_data(&self, entry: &FileNode) -> Vec<u8> {
        let file = fs::File::open(&self.cache_path).unwrap();
        let mut reader = BufReader::new(file);
        reader
            .seek(SeekFrom::Start(entry.cache_offset as u64))
            .unwrap();
        let mut buf = vec![0; entry.compressed_size as usize];
        // buf.fill(value);
        reader.read_exact(&mut buf).unwrap();

        buf
    }

    pub fn print_files(&self) {
        let root = self.dir_tree.root.read().unwrap();
        for ele in root.files.iter() {
            println!("{}", ele.read().unwrap().name);
        }
    }
}
