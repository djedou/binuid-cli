use crate::compiler::{llvm_compiler::Rule, BuildFrom};

#[derive(Debug)]
pub enum DLLStorageClass {
    None,
    Export,
	Import
}

impl BuildFrom for DLLStorageClass {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> DLLStorageClass {
        match pair.clone().into_inner().next() {
            Some(inner_pair) => {
                match inner_pair.as_rule() {
                    Rule::Dllexport => DLLStorageClass::Export,
                    Rule::Dllimport => DLLStorageClass::Import,
                    _ => DLLStorageClass::None
                }
            },
            None => DLLStorageClass::None
        }
    }
}