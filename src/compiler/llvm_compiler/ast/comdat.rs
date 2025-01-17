use crate::compiler::{llvm_compiler::Rule, BuildFrom};
use binuid_shared_wasm::ast_bits::composes::Comdat;



impl BuildFrom for Comdat {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> Comdat {
        match pair.clone().into_inner().next() {
            Some(inner_pair) => {
                match inner_pair.as_rule() {
                    Rule::ComdatName => Comdat::Name {
                        name: String::build_from(&inner_pair)
                    },
                    _ => Comdat::None
                }
            },
            None => Comdat::None
        }
    }
}