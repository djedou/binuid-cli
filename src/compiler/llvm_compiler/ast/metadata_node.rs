
use crate::compiler::{llvm_compiler::Rule, BuildFrom};
use binuid_shared_wasm::ast_bits::composes::{MetadataNode, MetadataId};

impl BuildFrom for MetadataNode {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> MetadataNode {
        let mut metadata = MetadataNode::new();

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::MetadataId => {
                    metadata.id = MetadataId::build_from(&inner_pair);
                },
                _ => {}
            }
        }

        metadata
    }
}