use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct LunPack {
    pub package_name: String,
    pub package_dir : String,
    pub package_exe : String,
    pub libraries   : Vec<String>,
}

impl LunPack {
    pub fn new(name: String, dir: String, package_exe: String) -> Self {
        Self {
            package_name: name,
            package_dir:  dir,
            package_exe:  package_exe,
            libraries: vec![],
        }
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct HowlingPacks {
    pub version : String, 
    pub packages: Vec<LunPack>,
    pub hello   : String,
}

impl HowlingPacks {
    pub fn new() -> Self {
        Self {
            version: "v0.0.1".to_string(),
            packages: vec![],
            hello: "HI".to_string(),
        }
    }
}