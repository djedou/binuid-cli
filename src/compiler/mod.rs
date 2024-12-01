mod llvm_compiler;

use std::path::{Path, PathBuf};
use std::{env, process::{Command, Stdio}};
use binuid_shared_wasm::console::{info, error};
use std::{
    fs::File,
    io::Read
};
use crate::{get_duid_bin_build, get_duid_bin_lib_build, Result, gather_ll_files};
use llvm_compiler::{LlvmCompiler, BuildFrom};


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
        let libs_ids = self.get_libs_paths();
        for file in &self.files {
            match (
                file.as_path().extension().map_or(None, |m| m.to_str()),
                file.as_path().to_str().map_or(false, |d| d.contains("pages"))
            ) {
                (Some("rs"), true) => {
                    match self.compile_file(&file, &libs_ids) {
                        Ok(_) => {},
                        Err(e) => {
                            return Err(e);
                        }
                    }
                },
                _ => {}
            }
        }

        self.compile_bin_ll();

        Ok(())
    }

    fn compile_file(&self, path: &PathBuf, _libs_ids: &[(String, String)]) -> Result<()> {
        let agrs = get_duid_bin_build(path.to_str(), &self.lib_dependencies);
        Command::new("rustc").args(&agrs).stdout(Stdio::inherit()).status()?;
        Ok(())
    }

    fn compile_bin_ll(&self) {
        let Ok(mut current_dir) = env::current_dir() else {
            return;
        };
        current_dir.push("dist");
        current_dir.push("libs");

        let mut files = vec![];
        gather_ll_files(current_dir.as_path(), &mut files);
        files.iter().for_each(|file| {
            self.compile_llvm(file.as_path(), false);
        });
    }

    fn compile_lib_bin(&self) -> Result<()> {
        let agrs = get_duid_bin_lib_build(&self.name, &self.version, &self.lib_dependencies);
        Command::new("rustc").args(&agrs).stdout(Stdio::inherit()).status()?;
        let Ok(mut current_dir) = env::current_dir() else {
            return Ok(());
        };
        current_dir.push("dist");
        current_dir.push(format!("{}_v_{}.ll", self.name, self.version));
        self.compile_llvm(current_dir.as_path(), true);
        
        Ok(())
    }

    fn get_libs_paths(&self) -> Vec<(String, String)> {
        let mut names = vec![];
        self.lib_dependencies.iter().for_each(|d| {
            match d.contains("=") {
                true => {
                    let splited: Vec<_> = d.split("=").collect();
                    names.push((splited[0].to_string(), splited[1].to_string()));
                },
                false => {}
            }
        });

        names
    }

    fn compile_llvm(&self, path: &Path, is_lib: bool) {
        let mut llvm_compiler = LlvmCompiler::new();
        llvm_compiler.read(&path);
        match is_lib {
            true => {
                let name = format!("{}_v_{}", self.name, self.version);
                llvm_compiler.compile(&path, Some(&name));
            },
            false => {
                llvm_compiler.compile(&path, None);
            }
        }
    }
}