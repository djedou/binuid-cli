use crate::compiler::{llvm_compiler::Rule, BuildFrom};
use binuid_shared_wasm::ast_bits::composes::{MDTuple, MetadataId, MetadataDef};


impl BuildFrom for MetadataDef {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> MetadataDef {
        let mut metadata = MetadataDef::new();

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::MetadataId => {
                    metadata.id = MetadataId::build_from(&inner_pair);
                },
                Rule::Distinct => {
                    metadata.distinct = true;
                },
                Rule::MDTuple => {
                    metadata.md_tuple = MDTuple::build_from(&inner_pair);
                },
                _ => {}
            }
        }

        metadata
    }
}