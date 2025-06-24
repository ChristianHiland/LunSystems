use crate::json_manager::{read_json_howling_packs, write_json_howling_packs};
use crate::structs::{*};
use crate::funcs::helpers::{*};



pub fn list_packages(path: String) {
    let package_data = read_json_howling_packs(path);
    for package in package_data.packages {
        println!("Package:      {}\nDIR:      {}\nEXE:      {}\nDepends on:      {:#?}", package.package_name, package.package_dir, package.package_exe, package.libraries);
    }
}

pub fn init_package_manager(path: String) {
    let mut package_data = HowlingPacks::new();
    let howling_installPackage = LunPack::new("HowlingInstall".to_string(), "/bin".to_string(), "/bin/HowlingInstall".to_string());
    package_data.packages.push(howling_installPackage);
    write_json_howling_packs(path, package_data);
}

pub fn mount_packages_boot(uuid: String, path: String) {
    mount_disk_at_uuid(uuid, path);
}