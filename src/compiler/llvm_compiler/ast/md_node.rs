use crate::compiler::{llvm_compiler::Rule, BuildFrom};
use binuid_shared_wasm::ast_bits::composes::{MetadataId, MDTuple, MDString, MDNode};


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