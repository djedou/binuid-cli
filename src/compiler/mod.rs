use std::path::PathBuf;
use std::{env, process::{Command, Stdio}};
use binuid_shared_wasm::console::{info, error};
use binuid_shared::syn;
use std::{
    fs::File,
    io::Read
};
use crate::{get_duid_bin_build, get_duid_bin_lib_build, Result};

pub struct Compiler {
    pub name: String, 
    pub version: String,
    pub files: Vec<PathBuf>,
    pub lib_dependencies: Vec<String>,
    pub zip_dependencies: Vec<String>
}

impl Compiler {
    pub fn new(name: &str, version: &str, files: &[PathBuf], lib_deps: &[String], zip_deps: &[String]) -> Compiler {
        Compiler {
            name: name.to_string(),
            version: version.to_string(),
            files: files.to_owned(),
            lib_dependencies: lib_deps.to_owned(),
            zip_dependencies: zip_deps.to_owned()

        }
    }

    pub fn compile(&self) -> Result<()> {
        info!("compiling...");
        self.compile_lib_bin()?;
        let mut errors_count = 0; 
        for file in &self.files {
            match (
                file.as_path().extension().map_or(None, |m| m.to_str()),
                file.as_path().to_str().map_or(false, |d| d.contains("pages"))
            ) {
                (Some("rs"), true) => {
                    match self.compile_file(&file) {
                        Ok(_) => {},
                        Err(e) => {
                            return Err(e);
                        }
                    }
                },
                _ => {}
            }
        }

        Ok(())
    }

    pub fn compile_file(&self, path: &PathBuf) -> Result<()> {
        let agrs = get_duid_bin_build(path.to_str(), &self.lib_dependencies);
        Command::new("rustc").args(&agrs).stdout(Stdio::inherit()).status()?;
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let ast = syn::parse_file(&content)?;
        println!("ast: {:#?}", ast.items.len());

        Ok(())
    }

    pub fn compile_lib_bin(&self) -> Result<()> {
        let agrs = get_duid_bin_lib_build(&self.name, &self.version, &self.lib_dependencies);
        Command::new("rustc").args(&agrs).stdout(Stdio::inherit()).status()?;
        Ok(())
    }
}