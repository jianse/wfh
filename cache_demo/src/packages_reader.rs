use std::path::Path;

use crate::{
    PackageTrioType, PackagesBin, package::Package, package_collection::PackageCollection,
};

#[derive(Debug)]
pub struct PackagesReader {
    pkg_names: Vec<String>,
    // path: PathBuf,
    pkgs_collection: PackageCollection,
    pkgs_bin: PackagesBin,
    // pkgs: Vec<Package>,
}

impl PackagesReader {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let path = path.as_ref();

        let mut s = PackagesReader {
            pkg_names: Vec::new(),
            pkgs_collection: PackageCollection::new(path.to_path_buf()),
            pkgs_bin: PackagesBin {
                entity_map: todo!(),
                is_initialized: todo!(),
                version: todo!(),
            },
        };
        s.pkg_names = PackagesReader::get_pkg_names(&s.pkgs_collection);
        s
    }

    fn get_pkg_names(pkg_dir: &PackageCollection) -> Vec<String> {
        pkg_dir.into_iter().map(|p| p.name.to_owned()).collect()
    }

    pub fn initialize_packages_bin(&mut self) {
        if self.pkgs_bin.is_initialized() {
            return;
        }
        let pkg_misc = self.pkgs_collection.get_package_mut("Misc").unwrap();
        let pair = pkg_misc.get_pair_mut(PackageTrioType::H).unwrap();
        pair.read_toc();
        let packages_raw = pair.get_data_and_decompress("Packages.bin");
        let reader = std::io::Cursor::new(packages_raw);
        self.pkgs_bin.initialize(reader)
        // self.pkgs_bin = PackagesBin::new(self.pkgs_collection.pkg_dir.clone());
    }

    pub fn get_package(&self, package_name: &str) -> Option<&Package> {
        self.pkgs_collection.get_package(package_name)
    }
}
