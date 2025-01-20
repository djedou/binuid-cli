/*
mod builders;
mod dependencies;
mod files;
mod zip_files;

pub(crate) use builders::*;
pub(crate) use dependencies::*;
pub(crate) use files::*;
pub(crate) use zip_files::*;
*/
use binuid_shared_wasm::{
    serde::{self, Deserialize, Serialize},
    toml::{Table, de::Error, from_str, to_string_pretty}
};
use std::{
    fs, env,
    path::{Path, PathBuf},
    fs::File
};
use std::io::{Read, Write};
/*
use binuid_shared::zip;
*/

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
#[serde(crate = "self::serde")]
pub(crate) struct BinuidPackage {
    pub(crate) package: Package,
    pub(crate) dependencies: Option<Table>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
#[serde(crate = "self::serde")]
pub(crate) struct Package {
    pub(crate) name: String,
    pub(crate) mode: Option<String>,
    pub(crate) version: Option<String>,
    pub(crate) authors: Option<Vec<String>>,
    pub(crate) members: Option<Vec<String>>
}

/*
#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(crate = "self::serde")]
pub(crate) struct Workspace {
    pub(crate) name: String,
    pub(crate) authors: Vec<String>,
}
*/

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
#[serde(crate = "self::serde")]
pub(crate) struct Dependency {
    pub(crate) name: String,
    pub(crate) version: Option<String>,
    pub(crate) path: Option<String>
}

pub(crate) fn read_binuid_config(name: &str) -> Result<BinuidPackage, Error> {
    let Ok(mut current_dir) = env::current_dir() else {
        return Ok(BinuidPackage::default());
    };
    current_dir.push(name);
    current_dir.push("binuid.toml");
    let Ok(mut file) = fs::File::open(&current_dir) else {
        return Ok(BinuidPackage::default());
    };
    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents);

    let binuid_config: BinuidPackage = from_str(&contents)?;
    Ok(binuid_config)
}

pub(crate) fn write_binuid_config(name: &str, config: BinuidPackage) {
    let Ok(mut current_dir) = env::current_dir() else {
        return;
    };
    current_dir.push(name);
    current_dir.push("binuid.toml");
    let Ok(mut file) = fs::File::create(&current_dir) else {
        return;
    };
    let Ok(contents) = to_string_pretty(&config) else {
        return;
    };
    let _ = file.write_all(&contents.as_bytes());
}