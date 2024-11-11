mod ast;
mod parser;

pub use ast::*;
pub use parser::*;
use std::path::Path;

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

    pub fn compile(&self, lib_name: Option<&str>) {
        // if lib_name = Some(name) then save the output in /dist/release/name/
        // if lib_name = None then save the output in /dist/release/
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