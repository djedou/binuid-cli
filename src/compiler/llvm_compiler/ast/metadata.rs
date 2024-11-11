use super::{TypeValue, MDString, MDTuple, MetadataId};
use crate::compiler::{llvm_compiler::Rule, BuildFrom};


#[derive(Debug)]
pub enum Metadata {
    None,
    TypeValue {
        type_value: TypeValue
    },
    MDString {
        data: MDString
    },
    MDTuple {
        tuple: MDTuple
    },
    MetadataId {
        id: MetadataId
    }
}

impl BuildFrom for Metadata {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> Metadata {
        match pair.clone().into_inner().next() {
            Some(inner_pair) => {
                match inner_pair.as_rule() {
                    Rule::TypeValue => Metadata::TypeValue {
                        type_value: TypeValue::build_from(&inner_pair)
                    },
                    Rule::MDString => Metadata::MDString {
                        data: MDString::build_from(&inner_pair)
                    },
                    Rule::MDTuple => Metadata::MDTuple {
                        tuple: MDTuple::build_from(&inner_pair)
                    },
                    Rule::MetadataId => Metadata::MetadataId {
                        id: MetadataId::build_from(&inner_pair)
                    },
                    _ => Metadata::None
                }
            },
            None => Metadata::None
        }
    }
}