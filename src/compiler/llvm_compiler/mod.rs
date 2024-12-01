mod ast;
mod parser;

pub use ast::*;
pub use parser::*;
use std::{env, fs::File, path::Path};
use std::collections::HashSet;
use std::io::Write;


pub struct LlvmCompiler {
    pub ast: LlvmAst
}

impl LlvmCompiler {
    pub fn new() -> LlvmCompiler {
        LlvmCompiler {
            ast: LlvmAst::new()
        }
    }

    pub fn read(&mut self, path: &Path) {
        self.ast = parse_ll(&std::fs::read_to_string(&path).unwrap());
    }

    pub fn compile(&self, path: &Path, lib_name: Option<&str>) {
        self.get_component_functions(&path);
        // if lib_name = Some(name) then save the output in /dist/release/name/
        // if lib_name = None then save the output in /dist/release/
    }

    fn get_component_functions(&self, path: &Path) {
        let mut component_set: HashSet<String> = HashSet::new();

        let functions: Vec<_> = self.ast.items.iter().filter(|item| match item.top_level_entity {
            TopLevelEntity::FunctionDef {def: _} => item.comments.iter().any(|a| {
                let seg: Vec<_> = a.value.split("::").collect();
                if seg.len() == 3 {
                    let _ = component_set.insert(seg[1].to_string());
                }

                (a.value.ends_with("view")
                || a.value.ends_with("update")
                || a.value.ends_with("subscribe")
                || a.value.ends_with("toast"))
                && seg.len() == 3
            }
            ),
            _ => false
        })
        .collect();
        
        let path_segments = path_segment(&path);;
        if functions.len() > 0 && path_segments.len() > 0 {
            println!("compiling path: {:#?}", path);

            component_set.iter().for_each(|comp| {
                // compile_component should return compiled component that we need to save
                match self.compile_component(&comp, &functions) {
                    true => {
                        let Ok(mut current_dir) = env::current_dir() else {
                            return;
                        };
                        current_dir.push("dist");
                        path_segments.iter().for_each(|p| current_dir.push(p));
                        current_dir.push(&format!("{comp}.bid"));
                        match File::options().read(true).write(true).create(true).open(current_dir.clone()) {
                            Ok(mut file) => {
                                let _ = file.write_all(b"Hello, world!");

                            },
                            Err(e) => {
                                println!("{:#?}", current_dir.as_path());
                                println!("{e:#?}");
                            }
                        }
                    },
                    false => {}
                }

            })
        }
    }

    fn compile_component(&self, name: &str, items: &[&Item]) -> bool {
        let mut test = false;
        // compile view
        match items.iter().find(|item| match item.top_level_entity {
            TopLevelEntity::FunctionDef {def: _} => 
                item.comments.iter().any(|a| a.value.ends_with(&format!("{name}::view"))),
            _ => false
        }) {
            Some(view) => {
                println!("compiling view: {:#?}", view);
                test = true;
            },
            None => {}
        }

        // compile update
        match items.iter().find(|item| match item.top_level_entity {
            TopLevelEntity::FunctionDef {def: _} => 
                item.comments.iter().any(|a| a.value.ends_with(&format!("{name}::update"))),
            _ => false
        }) {
            Some(update) => {
                println!("compiling update: {:#?}", update);
                test = true;
            },
            None => {}
        }

        // compile subscribe
        match items.iter().find(|item| match item.top_level_entity {
            TopLevelEntity::FunctionDef {def: _} => 
                item.comments.iter().any(|a| a.value.ends_with(&format!("{name}::subscribe"))),
            _ => false
        }) {
            Some(subscribe) => {
                println!("compiling subscribe: {:#?}", subscribe);
                test = true;
            },
            None => {}
        }

        // compile toast
        match items.iter().find(|item| match item.top_level_entity {
            TopLevelEntity::FunctionDef {def: _} => 
                item.comments.iter().any(|a| a.value.ends_with(&format!("{name}::toast"))),
            _ => false
        }) {
            Some(toast) => {
                println!("compiling toast: {:#?}", toast);
                test = true;
            },
            None => {}
        }

        test
    }
}


pub trait BuildFrom {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> Self;
}

impl BuildFrom for String {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> String {
        pair.as_str().trim().trim_matches('\"').to_string()
    }
}

fn path_segment(path: &Path) -> Vec<String> {
    let binding = path.to_string_lossy();
    let segments: Vec<_> = binding.split("dist").collect();
    if segments.len() == 2 {
        let mut needed = segments[1].replace(".ll", "");
        let seg: Vec<_> = needed.split("\\").collect();
        seg.iter().filter(|d| d.len() > 0).map(|d| d.to_string()).collect()
    }
    else {
        vec![]
    }
}