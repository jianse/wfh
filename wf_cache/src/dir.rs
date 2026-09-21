use std::ffi::CStr;
use std::fmt::Debug;
use std::fmt::Display;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::RwLock;

use binrw::BinRead;
use binrw::helpers::until_eof;
use binrw::io::BufReader;

// #[binrw]
#[derive(Debug, BinRead)]
#[br(little, magic = b"\x4e\xc6\x67\x18")]
struct Toc {
    version: u32,

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
pub struct DirNode {
    name: String,
    parent: Option<Arc<RwLock<DirNode>>>,
    pub dirs: Vec<Arc<RwLock<DirNode>>>,
    pub files: Vec<Arc<RwLock<FileNode>>>,
    dups: Vec<Arc<RwLock<FileNode>>>,
    index: usize,
}

impl DirNode {
    pub fn full_name(&self) -> String {
        if let Some(p) = self.parent.as_ref() {
            format!("{}/{}", p.read().unwrap().full_name(), self.name)
        } else {
            self.name.clone()
        }
    }
}

pub struct FileNode {
    pub name: String,
    parent: Option<Arc<RwLock<DirNode>>>,
    pub cache_offset: i64,
    timestamp: u64,
    pub compressed_size: u32,
    pub length: u32,
    index: usize,
}

impl FileNode {
    pub fn full_name(&self) -> String {
        if let Some(p) = self.parent.as_ref() {
            format!("{}/{}", p.read().unwrap().full_name(), self.name)
        } else {
            self.name.clone()
        }
    }
}

impl Debug for FileNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FileNode")
            .field("name", &self.name)
            .field("cache_offset", &self.cache_offset)
            .field("timestamp", &self.timestamp)
            .field("compressed_size", &self.compressed_size)
            .field("length", &self.length)
            .field("index", &self.index)
            .finish()
    }
}

#[derive(Debug)]
pub struct DirTree {
    toc_path: PathBuf,
    dirs: Vec<Arc<RwLock<DirNode>>>,
    pub(crate) files: Vec<Arc<RwLock<FileNode>>>,
    dups: Vec<Arc<RwLock<FileNode>>>,
    // any tree walk shoud start from root
    pub root: Arc<RwLock<DirNode>>,
    has_read: bool,
    dups_count: i64,
}

impl DirTree {
    pub(crate) fn new(toc_path: PathBuf) -> Self {
        let root = Arc::new(RwLock::new(DirNode {
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

    fn get_dir(&self, path: Option<String>) -> Option<Arc<RwLock<DirNode>>> {
        if let Some(path) = path {
            let mut current = self.root.clone();
            for part in path.split("/") {
                let part = part.trim();
                let find = {
                    let borrowed_current = current.read().unwrap();
                    borrowed_current
                        .dirs
                        .iter()
                        .find(|d| d.read().unwrap().name == part)
                        .cloned()
                };
                match find {
                    Some(d) => current = d,
                    None => return None,
                }
            }

            Some(current)
        } else {
            Some(self.root.clone())
        }
    }

    pub(crate) fn read_toc_file(&mut self) {
        let mut file = fs::File::open(&self.toc_path).unwrap();

        let mut buf = BufReader::new(&mut file);
        // let time = Instant::now();
        let toc = Toc::read(&mut buf).unwrap();

        for (i, entry) in toc.entries.iter().enumerate() {
            if entry.cache_offset == -1 {
                let dir = Arc::new(RwLock::new(DirNode {
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
                    e.write().unwrap().dirs.push(dir);
                    // e.dirs.push(dir);
                }
            } else if entry.timestamp == 0 {
                self.dups_count += 1;
                continue;
            } else {
                let file = Arc::new(RwLock::new(FileNode {
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
                    e.write().unwrap().files.push(file);
                }
            }
        }

        self.has_read = true;
    }

    pub fn get_file_entry(&self, name: &str) -> Option<Arc<RwLock<FileNode>>> {
        let path = name.rsplit_once("/");
        match path {
            Some((dir, file)) => {
                let dir = self.get_dir(Some(dir.to_owned())).unwrap();
                let borrowed_dir = dir.read().unwrap();
                let file = borrowed_dir
                    .files
                    .iter()
                    .find(|f| f.read().unwrap().name == file);
                file.cloned()
            }
            None => {
                let borrowed_root = self.root.read().unwrap();
                let file = borrowed_root
                    .files
                    .iter()
                    .find(|f| f.read().unwrap().name == name);
                file.cloned()
            }
        }
    }
}
