use crate::compiler::{llvm_compiler::Rule, BuildFrom};
use binuid_shared_wasm::ast_bits::composes::{MDField, MDTuple};


impl BuildFrom for MDTuple {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> MDTuple {
        let mut metadata = MDTuple::new();

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::MDField => {
                    metadata.fields.push(MDField::build_from(&inner_pair));
                },
                _ => {}
            }
        }

        metadata
    }
}