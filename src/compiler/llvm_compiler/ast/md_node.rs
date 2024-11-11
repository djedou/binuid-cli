use super::{MDTuple, MetadataId, MDString};
use crate::compiler::{llvm_compiler::Rule, BuildFrom};

#[derive(Debug)]
pub enum MDNode {
    None,
    Tuple {
        value: MDTuple
    },
	Id {
        id: MetadataId
    },
    String {
        data: MDString
    }
}



impl BuildFrom for MDNode {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> MDNode {
        match pair.clone().into_inner().next() {
            Some(inner_pair) => {
                match inner_pair.as_rule() {
                    Rule::MDTuple => MDNode::Tuple {
                        value: MDTuple::build_from(&inner_pair)
                    },
                    Rule::MetadataId => MDNode::Id {
                        id: MetadataId::build_from(&inner_pair)
                    },
                    Rule::MDString => MDNode::String {
                        data: MDString::build_from(&inner_pair)
                    },
                    _ => MDNode::None
                }
            },
            None => MDNode::None
        }
    }
}