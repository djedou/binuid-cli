use crate::compiler::{llvm_compiler::Rule, BuildFrom};
use binuid_shared_wasm::ast_bits::simples::DLLStorageClass;

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