use std::{collections::BTreeMap, path::PathBuf};

use crate::Package;

#[derive(Debug)]
pub struct PackageCollection {
    pkg_dir: PathBuf,
    pkg_map: BTreeMap<String, usize>,
    pkgs: Vec<Package>,
}

impl PackageCollection {
    pub fn new(pkg_dir: PathBuf) -> Self {
        let mut inst = Self {
            pkg_dir,
            pkg_map: BTreeMap::new(),
            pkgs: Vec::new(),
        };
        inst.load_packages();
        inst
    }

    pub fn get_package(&self, package_name: &str) -> Option<&Package> {
        self.pkg_map
            .get(package_name)
            .map(|index| &self.pkgs[*index])
    }

    pub fn get_package_mut(&mut self, package_name: &str) -> Option<&mut Package> {
        self.pkg_map
            .get(package_name)
            .map(|index| &mut self.pkgs[*index])
    }

    fn load_packages(&mut self) {
        for file in self.pkg_dir.read_dir().unwrap() {
            let file = file.unwrap();
            if file.file_type().unwrap().is_file() {
                let file_name = file.file_name();
                let file_name = file_name.to_string_lossy();
                let start = file_name.find(".");
                let end = file_name.rfind(".");
                if let Some(start) = start
                    && let Some(end) = end
                    && start < end
                {
                    let package_name = &file_name[start + 1..end];
                    if !self.pkg_map.contains_key(package_name) {
                        self.load_package(package_name);
                    }
                }
            }
        }
    }

    fn load_package(&mut self, package_name: &str) {
        self.pkg_map
            .insert(package_name.to_string(), self.pkgs.len());
        self.pkgs
            .push(Package::new(self.pkg_dir.clone(), package_name.to_string()));
    }
}

impl<'a> IntoIterator for &'a PackageCollection {
    type Item = &'a Package;

    type IntoIter = std::slice::Iter<'a, Package>;

    fn into_iter(self) -> Self::IntoIter {
        // todo!()
        self.pkgs.iter()
    }
}
