use std::{env, fs, path::PathBuf, process::{Command, Stdio}, io::{Read, Write}};
use binuid_shared_wasm::{serde_json::json, console::info};
use binuid_shared::{
    walkdir::WalkDir,
    quote::{quote, ToTokens},
    syn,
    proc_macro2::TokenTree
};
use binuid_compiler::Compiler;
use binuid_shared::walkdir::DirEntry;

use crate::{
    Metadata,
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
                    extend_code("./lib", false);
                    let _ = compile_lib(
                        &config.package.name.replace("-", "_"), 
                        &config.package.version.as_deref().map_or("0_0_0".to_owned(), |ver| ver.replace(".", "_")),
                        &deps_cmds
                    );
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
                    
                    extend_code("./app", true);
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
                    deps_cmds.extend_from_slice(&["--extern".to_owned(), format!("{name}=dist/lib{name}_v_{version}.rlib")]);
                    
                    let compiler = Compiler::new(&name, &version, &deps_cmds);
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

/*
fn get_bin_skip_files() -> Vec<PathBuf> {
    
    let Ok(current_dir) = env::current_dir() else {
        return vec![];
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

    files
}
*/
fn extend_code(base: &str, is_bin: bool) {
    let mut metadatas: Vec<Metadata> = vec![];

    for entry in WalkDir::new(base).into_iter().flatten() {
        if entry.clone().file_type().is_file() {
            let Ok(mut file) = fs::File::open(&entry.clone().into_path()) else {
                return;
            };
            let mut contents = String::new();
            let _ = file.read_to_string(&mut contents);
            let Ok(mut ast) = syn::parse_file(&contents) else {
                return;
            };

            let mut component_args: Vec<_> = vec![];

            ast.items.iter().for_each(|item| {
                match item {
                    syn::Item::Macro(macros) => {
                        match macros.mac.path.get_ident() {
                            Some(ident) => {
                                match &ident.to_string() == "component" {
                                    true => {
                                        let args: Vec<_> = macros.mac.tokens.clone().into_iter().filter(|st|  match st {
                                            TokenTree::Punct(_) | TokenTree::Literal(_) => false,
                                            TokenTree::Ident(ident) => &ident.to_string() != "Some",
                                            _ => true
                                        }).
                                        map(|st| {
                                            match st {
                                                TokenTree::Ident(ident) => ident.to_string(),
                                                TokenTree::Group(group) => {
                                                    match group.stream().into_iter().next() {
                                                        Some(TokenTree::Ident(ident)) => format!("Some({})", ident.to_string()),
                                                        _ => String::with_capacity(0)
                                                    }
                                                },
                                                _ => String::with_capacity(0)
                                            }
                                        })
                                        .collect();
                                        let mut path_components = get_path_components(entry.clone(), is_bin);
                                        path_components.retain(|d| d != ".");
                                        metadatas.push(Metadata {
                                            path: path_components,
                                            component_struct: args[0].clone(),
                                            component_function: args[1].clone()
                                        });
                                        component_args.push(args);
                                    },
                                    false => {}
                                }
                            },
                            None => {}
                        }
                    },
                    _ => {}
                }
            });

            ast.items.retain(|item| {
                match item {
                    syn::Item::Macro(macros) => {
                        match macros.mac.path.get_ident() {
                            Some(ident) => &ident.to_string() != "component",
                            None => true
                        }
                    },
                    _ => true
                }
            });

            let mut code_segments: Vec<String> = vec![];
            code_segments.push(ast.to_owned().into_token_stream().to_string());
            code_segments.extend_from_slice(
                &component_args.iter()
                .map(|args| component_template(args))
                .collect::<Vec<_>>()
            );
            
            let output = code_segments.join("\r");

            let mut path_components: Vec<_> = get_path_components(entry.clone(), is_bin);
            path_components.retain(|seg| seg != ".");

            write_new_page_code_config(&output, &path_components);
        }
    }
    
    write_metada(&metadatas, !is_bin);
}

fn get_path_components(entry: DirEntry, is_bin: bool) -> Vec<String> {
    entry.into_path().clone().components().map(|comp| match format!("{:#?}", comp.as_os_str()).as_ref() {
        "\"lib\"" => if is_bin {"lib".to_string()} else {"dist".to_string()},
        "\"app\"" => if is_bin {"dist".to_string()} else {"app".to_string()},
        a => a.replace("\"", "").to_string()
    }).collect::<Vec<String>>()
}

fn write_metada(metadatas: &[Metadata], is_bin: bool) {
    let content = json!(metadatas);
    
    let Ok(mut current_dir) = env::current_dir() else {
        return;
    };

    current_dir.push("dist");
    if !is_bin {
        current_dir.push("lib"); 
    }
    current_dir.push("metadata.json");

    let Ok(mut file) = fs::File::create(&current_dir) else {
        return;
    };
    let _ = file.write_all(&content.to_string().as_bytes());
}

fn write_new_page_code_config(content: &str, path_components: &[String]) {
    let Ok(mut current_dir) = env::current_dir() else {
        return;
    };
    
    path_components.iter().for_each(|seg| {
        current_dir.push(seg);
    });
    let Some(parent) = current_dir.as_path().parent() else {
        return;
    };
    let _ = fs::create_dir_all(&parent);
    let Ok(mut file) = fs::File::create(&current_dir) else {
        return;
    };
    let _ = file.write_all(&content.as_bytes());
}

fn component_template(args: &[String]) -> String {
    let comp = &args[0];
    let name = &args[1];
    let model = &args[2];  
    let msg = &args[3];
    let view = &args[4];
    let update = &args[5];
    let subscribe = &args[6];
    let toast = &args[7];
    /*
    #################################################################################################################
    // the content of this must be exactly the same as the component macros in binuid-libs/lib/binuid_std.rs
    #################################################################################################################
    */
    format!(r#"
pub struct {comp};

impl {comp} {{
    pub fn view(
        model: &{model},
        props: &[(&'static str, binuid_std::components::PropValue::<{msg}>)],
        children: &[binuid_std::components::Node<{model}, {msg}>]
    ) -> binuid_std::components::Node<{model}, {msg}> {{
        let new_view: Option<fn(
            model: &{model}, 
            props: &[(&'static str, binuid_std::components::PropValue::<{msg}>)],
            children: &[binuid_std::components::Node<{model}, {msg}>]
        ) -> binuid_std::components::Node<{model}, {msg}>> = {view};
        
        match new_view {{
            Some(v) => v(model, props, children),
            None => {{
                let mut new_props = std::collections::HashMap::new();
                for (key, value) in props {{
                    new_props.insert(key.to_string(), value.clone());
                }}
                let mut new_children = vec![];
                let len = children.len();
                for i in 0..len {{
                    new_children.push(children[i].clone());
                }}
                let mut node = binuid_std::components::Node::<{model}, {msg}>::default();
                node.tag = "{name}".to_string();
                node.model = <{model}>::default();
                node.props = new_props;
                node.children = new_children;

                node
            }}
        }}
    }}

    pub fn update(
        model: &mut {model}, 
        props: &mut binuid_std::components::Props<{msg}>, 
        msg: &{msg}
    ) -> binuid_std::components::Cmd<{msg}> {{
        let new_update: Option<fn(
            model: &mut {model}, 
            props: &mut binuid_std::components::Props<{msg}>, 
            msg: &{msg}
        ) -> binuid_std::components::Cmd<{msg}>> = {update};
        
        match new_update {{
            Some(u) => u(model, props, msg),
            None => binuid_std::components::Cmd::<{msg}>::None
        }}
    }}

    pub fn subscribe(
        model: &{model}, 
        props: &binuid_std::components::Props<{msg}>
    ) -> binuid_std::components::Sub<{msg}> {{
        let new_subscribe: Option<fn(
            model: &{model}, 
            props: &binuid_std::components::Props<{msg}>
        ) -> binuid_std::components::Sub<{msg}>> = {subscribe};
        
        match new_subscribe {{
            Some(s) => s(model, props),
            None => binuid_std::components::Sub::<{msg}>::None
        }}
    }}

    pub fn toast(
        model: &{model},
        props: &[(&'static str, binuid_std::components::PropValue::<{msg}>)],
        children: &[binuid_std::components::Node<{model}, {msg}>]
    ) -> binuid_std::components::Node<{model}, {msg}> {{
        let new_toast: Option<fn(
            model: &{model}, 
            props: &[(&'static str, binuid_std::components::PropValue::<{msg}>)],
            children: &[binuid_std::components::Node<{model}, {msg}>]
        ) -> binuid_std::components::Node<{model}, {msg}>> = {toast};
        
        match new_toast {{
            Some(t) => t(model, props, children),
            None => {{
                let mut new_props = std::collections::HashMap::new();
                for (key, value) in props {{
                    new_props.insert(key.to_string(), value.clone());
                }}
                let mut new_children = vec![];
                let len = children.len();
                for i in 0..len {{
                    new_children.push(children[i].clone());
                }}
                let mut node = binuid_std::components::Node::<{model}, {msg}>::default();
                node.tag = "toast-{name}".to_string();
                node.model = <{model}>::default();
                node.props = new_props;
                node.children = new_children;

                node
            }}
        }}
    }}

}}

pub fn {name}(
    model: &{model},
    props: &[(&'static str, binuid_std::components::PropValue::<{msg}>)],
    children: &[binuid_std::components::Node<{model}, {msg}>]
) -> binuid_std::components::Node<{model}, {msg}>
where {model}: std::fmt::Debug + Default + Clone,
        {msg}: std::fmt::Debug + Default + Clone
{{
    {comp}::view(model, props, children)
}}
"#)
}