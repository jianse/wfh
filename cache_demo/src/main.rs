use binrw::{BinRead, helpers::until_eof, io::BufReader};
use std::{
    cell::{Ref, RefCell},
    collections::HashMap,
    ffi::CStr,
    fmt::{Debug, Display},
    fs,
    io::{Read, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
    rc::Rc,
    time::Instant,
};
use strum::EnumIter;

use crate::{package::Package, package_collection::PackageCollection, packages_reader::PackagesReader};

mod package;
mod package_collection;
mod packages_reader;



#[derive(Debug)]
pub struct PackagesEntity {}

#[derive(Debug)]
pub struct PackagesBin {
    entity_map: HashMap<String, PackagesEntity>,
    pub is_initialized: bool,
    version: i32,
}

impl PackagesBin {
    pub fn initialize(&mut self, reader: impl Read + Seek) {
        if self.is_initialized {
            return;
        }
        let raw_entries = self.read_file(reader);

        self.is_initialized = true;
    }
    fn read_file(&mut self, reader: impl Read + Seek) {}
}

impl PackagesBin {
    pub fn new(path: PathBuf) -> Self {
        Self {
            entity_map: HashMap::new(),
            is_initialized: false,
            version: 0,
        }
    }

    pub fn is_initialized(&self) -> bool {
        // exists(&self.path().join("index.bin")).unwrap()
        todo!("Not implemented")
    }
}

#[derive(EnumIter)]
pub enum PackageTrioType {
    H,
    F,
    B,
}

#[derive(Debug)]
pub struct CachePair {
    toc_path: PathBuf,
    cache_path: PathBuf,
    dir_tree: DirTree,
    // size: usize,
    has_read_toc: bool,
}

impl CachePair {
    pub fn new(toc_path: PathBuf, cache_path: PathBuf) -> Self {
        Self {
            toc_path: toc_path.clone(),
            cache_path,
            dir_tree: DirTree::new(toc_path),
            has_read_toc: false,
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
        let borrowed_entry = entry.borrow();
        if borrowed_entry.compressed_size == borrowed_entry.length {
            return self.get_data(borrowed_entry);
        }

        // decompress
        todo!("Not implemented")

        // if entry
    }
    fn get_data(&self, entry: Ref<'_, FileNode>) -> Vec<u8> {
        let file = fs::File::open(&self.cache_path).unwrap();
        let mut reader = BufReader::new(file);
        reader
            .seek(SeekFrom::Start(entry.cache_offset as u64))
            .unwrap();
        let mut buf = Vec::with_capacity(entry.compressed_size as usize);
        reader.read_exact(&mut buf).unwrap();

        buf
    }
}

pub struct PackageReader {
    package: Package,
}

// #[binrw]
#[derive(Debug, BinRead)]
#[br(little, magic = b"\x4e\xc6\x67\x18")]
struct Toc {
    version: u32,
    // magic:
    #[br(parse_with = until_eof)]
    entries: Vec<RawTocEntry>,
}

// #[binrw]
// #[derive(Debug)]
#[derive(BinRead)]
#[br(little)]
struct RawTocEntry {
    cache_offset: i64,
    timestamp: u64,
    compressed_size: u32,
    length: u32,
    reserved: u32,
    parent_dir_index: u32,
    #[br(count = 64)]
    name: Vec<u8>,
}

impl RawTocEntry {
    pub fn get_name(&self) -> String {
        let name = CStr::from_bytes_until_nul(&self.name).unwrap();
        name.to_string_lossy().to_string()
    }
}

impl Debug for RawTocEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_string().as_str())
    }
}

impl Display for RawTocEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = CStr::from_bytes_until_nul(&self.name).map_err(|_e| std::fmt::Error)?;
        f.debug_struct("RawTocEntry")
            .field("cache_offset", &self.cache_offset)
            .field("timestamp", &self.timestamp)
            .field("compressed_size", &self.compressed_size)
            .field("length", &self.length)
            .field("reserved", &self.reserved)
            .field("parent_dir_index", &self.parent_dir_index)
            .field("name", &name)
            .finish()
    }
}

#[derive(Debug)]
struct DirNode {
    name: String,
    parent: Option<Rc<RefCell<DirNode>>>,
    dirs: Vec<Rc<RefCell<DirNode>>>,
    files: Vec<Rc<RefCell<FileNode>>>,
    dups: Vec<Rc<RefCell<FileNode>>>,
    index: usize,
}

#[derive(Debug)]
struct FileNode {
    name: String,
    parent: Option<Rc<RefCell<DirNode>>>,
    cache_offset: i64,
    timestamp: u64,
    compressed_size: u32,
    length: u32,
    index: usize,
}

#[derive(Debug)]
struct DirTree {
    toc_path: PathBuf,
    dirs: Vec<Rc<RefCell<DirNode>>>,
    files: Vec<Rc<RefCell<FileNode>>>,
    dups: Vec<Rc<RefCell<FileNode>>>,
    root: Rc<RefCell<DirNode>>,
    has_read: bool,
    dups_count: i64,
}

impl DirTree {
    fn new(toc_path: PathBuf) -> Self {
        let root = Rc::new(RefCell::new(DirNode {
            name: "".to_string(),
            parent: None,
            dirs: Vec::new(),
            files: Vec::new(),
            dups: Vec::new(),
            index: 0,
        }));
        Self {
            toc_path,
            root: root.clone(),
            dirs: vec![root],
            files: Vec::new(),
            dups: Vec::new(),
            has_read: false,
            dups_count: -1,
        }
    }

    fn get_dir(&self, path: &str) -> Option<Rc<RefCell<DirNode>>> {
        let mut current = self.root.clone();
        for part in path.split("/") {
            let find = {
                let borrowed_current = current.borrow();
                borrowed_current
                    .dirs
                    .iter()
                    .find(|d| d.borrow().name == part)
                    .cloned()
            };
            match find {
                Some(d) => current = d,
                None => return None,
            }
        }

        Some(current)
    }

    fn read_toc_file(&mut self) {
        let mut file = fs::File::open(&self.toc_path).unwrap();

        let mut buf = BufReader::new(&mut file);
        // let time = Instant::now();
        let toc = Toc::read(&mut buf).unwrap();

        for (i, entry) in toc.entries.iter().enumerate() {
            if entry.cache_offset == -1 {
                let dir = Rc::new(RefCell::new(DirNode {
                    name: entry.get_name(),
                    parent: self.dirs.get(entry.parent_dir_index as usize).cloned(),
                    dirs: Vec::new(),
                    files: Vec::new(),
                    dups: Vec::new(),
                    index: i,
                }));
                self.dirs.push(dir.clone());

                let e = self.dirs.get_mut(entry.parent_dir_index as usize);
                if let Some(e) = e {
                    e.borrow_mut().dirs.push(dir);
                    // e.dirs.push(dir);
                }
            } else {
                if entry.timestamp == 0 {
                    self.dups_count += 1;
                    continue;
                } else {
                    let file = Rc::new(RefCell::new(FileNode {
                        name: entry.get_name(),
                        parent: self.dirs.get(entry.parent_dir_index as usize).cloned(),
                        cache_offset: entry.cache_offset,
                        timestamp: entry.timestamp,
                        compressed_size: entry.compressed_size,
                        length: entry.length,
                        index: i,
                    }));
                    self.files.push(file.clone());
                    let e = self.dirs.get_mut(entry.parent_dir_index as usize);
                    if let Some(e) = e {
                        e.borrow_mut().files.push(file);
                    }
                }
            }
        }

        self.has_read = true;
    }

    pub fn get_file_entry(&self, name: &str) -> Option<Rc<RefCell<FileNode>>> {
        let path = name.rsplit_once("/");
        match path {
            Some((dir, file)) => {
                let dir = self.get_dir(dir).unwrap();
                let borrowed_dir = dir.borrow();
                let file = borrowed_dir.files.iter().find(|f| f.borrow().name == file);
                file.cloned()
            }
            None => {
                let borrowed_root = self.root.borrow();
                let file = borrowed_root.files.iter().find(|f| f.borrow().name == name);
                file.cloned()
            }
        }
    }
}

fn main() {
    // let cache_reader = CacheReader::new("F:/WeGameApps/rail_apps/Warframe(2000576)/Cache.Windows");
    // dbg!(cache_reader);
    let pr = PackagesReader::new("F:/WeGameApps/rail_apps/Warframe(2000576)/Cache.Windows");

    let p = pr.get_package("Misc_zh").unwrap();

    dbg!(p);
}

// #[derive(Debug)]
