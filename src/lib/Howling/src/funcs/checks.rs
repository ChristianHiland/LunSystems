use std::path::Path;

pub fn check_for_file(filepath: String) -> bool {
    let root_path = crate::get_root_path().unwrap();
    let full_path = root_path.join(filepath);
    let path = Path::new(full_path.to_str().unwrap());
    if path.exists() { true } else { false }
}

pub fn check_for_dir(path: String) -> bool {
    let root_path = crate::get_root_path().unwrap();
    let full_path = root_path.join(path);
    let path = Path::new(full_path.to_str().unwrap());
    if path.exists() { true } else { false }
}