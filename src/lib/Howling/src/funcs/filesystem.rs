use std::io::{self, Write};
use std::path::PathBuf;
use std::io::Error;
use std::fs::File;
use std::{env, fs};


pub fn get_root_path() -> Result<PathBuf, Error> {
    let current_dir = env::current_dir();
    match current_dir {
        Ok(path) => Ok(path),
        Err(error) => Err(error.into()),
    }
}

pub fn make_file(filename: String, option: &str) -> io::Result<File> {
    let mut file = File::create(filename.clone())?;
    println!("File '{}' created successfully!", filename);
    Ok(file)
}

pub fn delete_file(filename: String) -> io::Result<()> {
    fs::remove_file(filename)?;
    Ok(())
}