use crate::compiler::{llvm_compiler::Rule, BuildFrom};


#[derive(Debug)]
pub enum UnnamedAddr { 
    None,
    Local,
	NoLocal
}


impl BuildFrom for UnnamedAddr {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> UnnamedAddr {
        match pair.clone().into_inner().next() {
            Some(inner_pair) => {
                match inner_pair.as_rule() {
                    Rule::LocalUnnamedAddr => UnnamedAddr::Local,
                    Rule::NoLocalUnnamedAddr => UnnamedAddr::NoLocal,
                    _ => UnnamedAddr::None
                }
            },
            None => UnnamedAddr::None
        }
    }
}