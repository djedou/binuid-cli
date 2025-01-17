mod ast;
mod parser;

pub use ast::*;
pub use parser::*;
use std::{env, fs::{create_dir_all, File}, path::Path};
use std::collections::HashMap;
use std::io::Write;
use binuid_shared_wasm::vm::{Component, Frame};
use binuid_shared_wasm::ast_bits::composes::TopLevelEntity;


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
        let component_set = self.get_component_functions();
        component_set.iter().for_each(|(key, values)| {
            match !values.is_empty() {
                true => {
                    self.compile_component(&path, &key, &values);
                },
                false => {}
            }
        })
        // if lib_name = Some(name) then save the output in /dist/release/name/
        // if lib_name = None then save the output in /dist/release/
    }

    fn get_component_functions(&self) -> HashMap<String, Vec<String>> {
        let mut component_set: HashMap<String, Vec<String>> = HashMap::new();

        self.ast.items.iter().for_each(|item| match item.top_level_entity {
            TopLevelEntity::FunctionDef {def: _} => {
                item.comments.iter().for_each(|a| {
                    let seg: Vec<_> = a.value.split("::").collect();
                    match (a.value.ends_with("view")
                    || a.value.ends_with("update")
                    || a.value.ends_with("subscribe")
                    || a.value.ends_with("toast"))
                    && seg.len() == 3 {
                        true => {
                            match component_set.get_mut(seg[1]) {
                                Some(funcs) => {
                                    funcs.push(a.value.clone());
                                },
                                None => {
                                    component_set.insert(seg[1].to_string(), vec![a.value.clone()]);
                                }
                            }
                        },
                        false => {}
                    }
                });
            }
            _ => {}
        });
        component_set
    }

    fn compile_component(&self, path: &Path, component_name: &str, functions_names: &[String]) {
        //println!("compiling comp: {:#?}", component_name);
        let mut component = Component::default();

        let path_segments = path_segment(&path);
        let Ok(mut current_dir) = env::current_dir() else {
            return;
        };
        current_dir.push("dist");
        path_segments.iter().for_each(|p| {
            match p == "libs" {
                true => {
                    current_dir.push("components");
                },
                false => {
                    current_dir.push(p);
                }
            }
        });
        let Ok(_) = create_dir_all(current_dir.clone()) else {
            return;
        };


        // compile view
        match self.ast.items.iter().find(|item| match item.top_level_entity {
            TopLevelEntity::FunctionDef {def: _} => 
                item.comments.iter().any(|a| a.value.ends_with("view") && functions_names.iter().any(|f| f == &a.value)),
            _ => false
        }) {
            Some(view) => {
                component.view = self.compile_function(&view);
            },
            None => {}
        }

        // compile update
        match self.ast.items.iter().find(|item| match item.top_level_entity {
            TopLevelEntity::FunctionDef {def: _} => 
                item.comments.iter().any(|a| a.value.ends_with("update") && functions_names.iter().any(|f| f == &a.value)),
            _ => false
        }) {
            Some(update) => {
                component.update = self.compile_function(&update);
            },
            None => {}
        }

        // compile subscribe
        match self.ast.items.iter().find(|item| match item.top_level_entity {
            TopLevelEntity::FunctionDef {def: _} => 
                item.comments.iter().any(|a| a.value.ends_with("subscribe") && functions_names.iter().any(|f| f == &a.value)),
            _ => false
        }) {
            Some(subscribe) => {
                component.subscribe = self.compile_function(&subscribe);
            },
            None => {}
        }

        // compile toast
        match self.ast.items.iter().find(|item| match item.top_level_entity {
            TopLevelEntity::FunctionDef {def: _} => 
                item.comments.iter().any(|a| a.value.ends_with("toast") && functions_names.iter().any(|f| f == &a.value)),
            _ => false
        }) {
            Some(toast) => {
                component.toast = self.compile_function(&toast);
            },
            None => {}
        }
        
        // Save component
        current_dir.push(&format!("{component_name}.bid"));
        println!("djed path: {:#?}", current_dir.display());
        match File::options().read(true).write(true).create(true).open(current_dir.clone()) {
            Ok(mut file) => {
                let comp_string = format!("{component:#?}");
                let _ = file.write_all(&comp_string.into_bytes());
            },
            Err(e) => {
                println!("{:#?}", current_dir.as_path());
                println!("{e:#?}");
            }
        }
    }

    // this produce a Frame
    fn compile_function(&self, fun_item: &Item) -> Frame {
        let mut frame = Frame::default();
        fun_item.into_frame(&self.ast, &mut frame);

        frame
    }
}

pub trait BuildFrom {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> Self;
}


pub trait IntoFrame {
    fn into_frame(&self, ast: &LlvmAst, frame: &mut Frame);
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