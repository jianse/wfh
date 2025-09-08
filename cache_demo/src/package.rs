use std::{fs::exists, path::PathBuf};

use strum::IntoEnumIterator;

use crate::{CachePair, PackageTrioType};

#[derive(Debug)]
pub struct Package {
    pkg_dir: PathBuf,
    pub name: String,
    pkgs: [Option<CachePair>; 3],
    // version: String,
}

impl Package {
    pub fn new(path: PathBuf, package_name: String) -> Self {
        let mut s = Self {
            pkg_dir: path,
            name: package_name,
            pkgs: [None, None, None],
        };
        s.load_pkg_pairs();
        s
    }

    pub fn get_pair(&self, trio_type: PackageTrioType) -> Option<&CachePair> {
        self.pkgs[trio_type as usize].as_ref()
    }

    pub fn get_pair_mut(&mut self, trio_type: PackageTrioType) -> Option<&mut CachePair> {
        self.pkgs[trio_type as usize].as_mut()
    }

    fn load_pkg_pairs(&mut self) {
        for t in PackageTrioType::iter() {
            let pair = self.get_pair_path(&t);
            if exists(&pair.0).unwrap() && exists(&pair.1).unwrap() {
                self.pkgs[t as usize] = Some(CachePair::new(pair.0, pair.1))
            }
        }
    }
    fn get_pair_path(&self, trio_type: &PackageTrioType) -> (PathBuf, PathBuf) {
        match trio_type {
            PackageTrioType::H => (
                self.pkg_dir.join(format!("H.{}.toc", self.name)),
                self.pkg_dir.join(format!("H.{}.cache", self.name)),
            ),
            PackageTrioType::F => (
                self.pkg_dir.join(format!("F.{}.toc", self.name)),
                self.pkg_dir.join(format!("F.{}.cache", self.name)),
            ),
            PackageTrioType::B => (
                self.pkg_dir.join(format!("B.{}.toc", self.name)),
                self.pkg_dir.join(format!("B.{}.cache", self.name)),
            ),
        }
    }
}
