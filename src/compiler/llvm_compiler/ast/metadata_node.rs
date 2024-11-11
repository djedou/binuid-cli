use super::MetadataId;
use crate::compiler::{llvm_compiler::Rule, BuildFrom};

#[derive(Debug)]
pub struct MetadataNode { 
    pub id: MetadataId
}


impl MetadataNode {
    pub fn new() -> MetadataNode {
        MetadataNode {
            id: MetadataId::new()
        }
    }
}


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