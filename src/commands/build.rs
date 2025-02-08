use std::{env, fs, path::Path, process::{Command, Stdio}, io::{Read, Write}};
use binuid_shared_wasm::{serde_json::json, console::{error, info}};
use binuid_shared::{
    walkdir::WalkDir,
    quote::{quote, ToTokens},
    syn,
    proc_macro2::TokenTree
};
use binuid_compiler::Compiler;
use binuid_shared::walkdir::DirEntry;
use binuid_compiler::model::{PageFile, FileMetadata};
use std::ffi::OsStr;


use crate::{
    Result, read_binuid_config, read_dependencies_from_table, 
    get_dependency_path, gather_files, save_zip, extract_lib_from_zip, get_duid_lib_build,
    get_dependency_zip_path, get_duid_bin_lib_build
};

pub(crate) fn build() -> Result<()> {
    let config = read_binuid_config("")?;
    match config.package.mode.as_deref() {
        Some("lib") => {
            match config.dependencies {
                Some(dependencies) => {
                    let deps = read_dependencies_from_table(&dependencies);
                    let deps_cmds = deps.into_iter().map(|dep| {
                        //### This need to be moved to where you first add this dependency to the project ########
                        extract_lib_from_zip(&dep.name.replace("-", "_"), &dep.version.as_deref().map_or("0_0_0".to_owned(), |ver| ver.replace(".", "_")));
                        //### End #####
                        vec!["--extern".to_owned(), format!("{}={}", dep.name, get_dependency_path(&dep.name, dep.version.as_deref(), dep.path.as_deref()))]
                    })
                    .flatten()
                    .collect::<Vec<String>>();
                    let name = config.package.name.replace("-", "_");
                    let version = config.package.version.as_deref().map_or("0_0_0".to_owned(), |ver| ver.replace(".", "_"));
                    let _ = compile_lib(
                        &name, 
                        &version,
                        &deps_cmds
                    );
                    get_metadata("./lib", &format!("{}_v_{}.json", name, version));
                },
                None => {}
            }
            Ok(())
        },
        Some("bin") => {
            match config.dependencies {
                Some(dependencies) => {
                    let deps = read_dependencies_from_table(&dependencies);
                    // dependencies
                    let mut deps_files = vec![];
                    let mut deps_cmds = vec![];
                    
                    deps.iter().for_each(|d| {
                        //### This need to be moved to where you first add this dependency to the project ########
                            extract_lib_from_zip(&d.name.replace("-", "_"), &d.version.as_deref().map_or("0_0_0".to_owned(), |ver| ver.replace(".", "_")));
                        //### End #####
                        deps_files.push(get_dependency_zip_path(&d.name, d.version.as_deref(), d.path.as_deref()));
                        deps_cmds.extend_from_slice(&vec!["--extern".to_owned(), format!("{}={}", &d.name, get_dependency_path(&d.name, d.version.as_deref(), d.path.as_deref()))]);
                    });
                    
                    let name = config.package.name.replace("-", "_");
                    let version = match &config.package.version {
                        Some(ver) => ver.replace(".", "_"),
                        None => "0_0_0".to_string()
                    };
                    
                    let _ = compile_lib_bin(
                        &name, 
                        &version,
                        &deps_cmds
                    );
                    get_metadata("./app/lib", &format!("{}_v_{}.json", name, version));

                    deps_cmds.extend_from_slice(&["--extern".to_owned(), format!("{name}=dist/lib{name}_v_{version}.rlib")]);
                    
                    let mut compiler = Compiler::new(&name, &version, &deps_cmds);
                    compiler.compile();
                },
                None => {}
            }
            Ok(())
        },
        Some("ws") => {
            Ok(())
        },
        _ => {
            Ok(())
        }
    }
}

fn get_metadata(base: &str, name: &str) {
    let mut file_metadatas: Vec<FileMetadata> = vec![];
    
    for entry in WalkDir::new(base).into_iter().flatten() {
        if entry.clone().file_type().is_file() 
        &&  entry.clone().into_path().extension().and_then(OsStr::to_str).map_or(false, |d| d == "rs") {
            let mut page = PageFile::load(&entry);
            let file_metadata = page.get_file_metadatas();
            file_metadatas.push(file_metadata);
        }
    }

    write_metadata(&file_metadatas, &name)
}

fn write_metadata(metadatas: &[FileMetadata], name: &str) {
    let content = json!(metadatas);
    let Ok(mut current_dir) = env::current_dir() else {
        return;
    };
    current_dir.push("dist");
    current_dir.push(name);
    let Ok(mut file) = fs::File::create(&current_dir) else {
        return;
    };
    let _ = file.write_all(&content.to_string().as_bytes());
}

fn compile_lib_bin(name: &str, version: &str, deps_cmds: &[String]) -> Result<()> {
    Command::new("rustc")
        .args(&get_duid_bin_lib_build(&name, &version, &deps_cmds))
        .stdout(Stdio::inherit())
        .status()?;
    Ok(())
}

fn compile_lib(name: &str, version: &str, deps_cmds: &[String]) -> Result<()> {
    match Command::new("rustc")
        .args(get_duid_lib_build(&name, &version, &deps_cmds))
        .stdout(Stdio::inherit())
        .status()
    {
        Ok(_) => {
            let Ok(current_dir) = env::current_dir() else {
                return Ok(());
            };
            let mut current_dir_doc = current_dir.clone();
            current_dir_doc.push("doc");
            let mut current_dir_git = current_dir.clone();
            current_dir_git.push(".git");
            let mut current_dir_deps = current_dir.clone();
            current_dir_deps.push("deps");
            let skips = vec![
                format!("{}", current_dir_doc.display()),
                format!("{}", current_dir_git.display()),
                format!("{}", current_dir_deps.display())
            ];
            let root = format!("{}", current_dir.display());
            let mut files = vec![];
            gather_files(Some(skips.as_slice()), current_dir.as_path(), &mut files);
            let zip_name = format!("{}_v_{}.zip", &name, version); 
            let _ = save_zip(&root, &zip_name, &files);
        },
        Err(err) => {
            println!("Err: {:#?}", err);
        }
    }

    Ok(())
}
