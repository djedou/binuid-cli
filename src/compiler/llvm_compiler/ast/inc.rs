use crate::compiler::{llvm_compiler::Rule, BuildFrom};
use binuid_shared_wasm::ast_bits::composes::{Inc, Ident};
use binuid_shared_wasm::ast_bits::values::Value;



impl BuildFrom for Inc {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> Inc {
        let mut item = Inc::new();

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::Value => {
                    item.value = Value::build_from(&inner_pair);
                },
                Rule::Type => {
                    item.local_ident = Ident::build_from(&inner_pair);
                },
                _ => {}
            }
        }

        item
    }
}