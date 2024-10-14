use std::{env, process::{Command, Stdio}};
use binuid_shared_wasm::console::info;
use crate::{
    compiler::Compiler, Result, read_binuid_config, read_dependencies_from_table, 
    get_dep_path, gather_files, save_zip, extract_lib_from_zip, get_duid_lib_build,
    get_dep_zip_path
};

pub(crate) fn build() -> Result<()> {
    //info!("building");
    //info!("building with info!");
    let config = read_binuid_config("")?;
    match (config.library.as_ref(), config.binary.as_ref(), config.workspace.as_ref()) {
        (Some(library), _, _) => {
            //info!("{:#?}", library);
            match config.dependencies {
                Some(dependencies) => {
                    let deps = read_dependencies_from_table(&dependencies);
                    let deps_cmds = deps.into_iter().map(|dep| {
                        vec!["--extern".to_owned(), format!("{}={}", dep.name, get_dep_path(&dep.name, dep.version.as_deref(), dep.path.as_deref()))]
                    })
                    .flatten()
                    .collect::<Vec<String>>();
                    match Command::new("rustc")
                        .args(get_duid_lib_build(
                            &library.name, 
                            &library.version.as_deref().map_or("0_0_0".to_owned(), |ver| ver.replace(".", "_")), 
                            &deps_cmds)
                        )
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
                            let zip_name = match &library.version {
                                Some(ver) => {
                                    format!("{}_v_{}.zip", &library.name.replace("-", "_"), ver.replace(".", "_"))
                                },
                                None => {
                                    format!("{}_v_0_0_0.zip", &library.name.replace("-", "_"))
                                }
                            };
                            let _ = save_zip(&root, &zip_name, &files);
                        },
                        Err(err) => {
                            println!("Err: {:#?}", err);
                        }
                    }
                },
                None => {}
            }
            Ok(())
        },
        (_, Some(binary), _) => {
            //info!("{:#?}", binary);
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
                        deps_files.push(get_dep_zip_path(&d.name, d.version.as_deref(), d.path.as_deref()));
                        deps_cmds.extend_from_slice(&vec!["--extern".to_owned(), format!("{}={}", &d.name, get_dep_path(&d.name, d.version.as_deref(), d.path.as_deref()))]);
                    });
                    
                    let Ok(current_dir) = env::current_dir() else {
                        return Ok(());
                    };
                    
                    let mut current_dir_doc = current_dir.clone();
                    current_dir_doc.push("doc");
                    let mut current_dir_git = current_dir.clone();
                    current_dir_git.push(".git");
                    let mut current_dir_deps = current_dir.clone();
                    current_dir_deps.push("deps");
                    let mut current_dir_pkg = current_dir.clone();
                    current_dir_pkg.push("pkg");
                    let mut current_dir_dist = current_dir.clone();
                    current_dir_dist.push("dist");
                    let mut current_dir_app_lib = current_dir.clone();
                    current_dir_app_lib.push("app");
                    current_dir_app_lib.push("lib");
                    let skips = vec![
                        format!("{}", current_dir_doc.display()),
                        format!("{}", current_dir_git.display()),
                        format!("{}", current_dir_deps.display()),
                        format!("{}", current_dir_pkg.display()),
                        format!("{}", current_dir_app_lib.display()),
                        format!("{}", current_dir_dist.display())
                    ];
                    let mut files = vec![];
                    gather_files(Some(skips.as_slice()), current_dir.as_path(), &mut files);
                    let version = match &binary.version {
                        Some(ver) => ver.replace(".", "_"),
                        None => "0_0_0".to_string()
                    };
                    let name = binary.name.replace("-", "_");
                    deps_cmds.extend_from_slice(&["--extern".to_owned(), format!("{}=dist/lib{name}_v_{}.rlib", name, version)]);
                    let compiler = Compiler::new(&name, &version, &files, &deps_cmds, &deps_files);
                    let _ = compiler.compile()?;
                },
                None => {}
            }
            Ok(())
        },
        (_, _, _) => {
            Ok(())
        }
    }
}
