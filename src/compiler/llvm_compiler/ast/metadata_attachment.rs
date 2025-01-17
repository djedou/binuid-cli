
use crate::compiler::{llvm_compiler::Rule, BuildFrom};
use binuid_shared_wasm::ast_bits::composes::{MetadataName, MDNode, MetadataAttachment};



impl BuildFrom for MetadataAttachment {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> MetadataAttachment {
        let mut metadata = MetadataAttachment {
            metadata_name: MetadataName::new(),
            md_node: MDNode::None
        };

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::MetadataName => {
                    metadata.metadata_name = MetadataName::build_from(&inner_pair);
                },
                Rule::MDNode => {
                    metadata.md_node = MDNode::build_from(&inner_pair);
                },
                _ => {}
            }
        }

        metadata
    }
}