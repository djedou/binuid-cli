use serde::{Deserialize, Serialize};
use toml::{Table, de::Error, from_str, to_string_pretty};
use std::{fs, env};
use crate::Mode;
use std::io::{Read, Write};

#[derive(Debug, Deserialize, Serialize, Default)]
pub(crate) struct BinuidConfig {
    pub(crate) package: Option<Package>,
    pub(crate) library: Option<Package>,
    pub(crate) binary: Option<Package>,
    pub(crate) workspace: Option<Workspace>,
    pub(crate) dependencies: Option<Table>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub(crate) struct Package {
    pub(crate) name: String,
    pub(crate) version: Option<String>,
    pub(crate) authors: Vec<String>
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub(crate) struct Workspace {
    pub(crate) name: String,
    pub(crate) authors: Vec<String>,
    pub(crate) members: Vec<String>
}

pub(crate) fn read_binuid_config(name: &str) -> Result<BinuidConfig, Error> {
    let Ok(mut current_dir) = env::current_dir() else {
        return Ok(BinuidConfig::default());
    };
    current_dir.push(name);
    current_dir.push("binuid.toml");
    let Ok(mut file) = fs::File::open(&current_dir) else {
        return Ok(BinuidConfig::default());
    };
    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents);

    let binuid_config: BinuidConfig = from_str(&contents)?;
    Ok(binuid_config)
}

pub(crate) fn write_binuid_config(name: &str, config: BinuidConfig) {
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