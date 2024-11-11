use super::{MetadataName, MDNode};
use crate::compiler::{llvm_compiler::Rule, BuildFrom};

#[derive(Debug)]
pub struct MetadataAttachment {
    pub metadata_name: MetadataName,
    pub md_node: MDNode
}


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