use super::Metadata;
use crate::compiler::{llvm_compiler::Rule, BuildFrom};
/*
#[derive(Debug)]
pub struct MDFields {
    pub list: Vec<MDField>
}
*/

#[derive(Debug)]
pub enum MDField {
    None,
    Null,
    Metadata {
        data: Metadata
    }
}


impl BuildFrom for MDField {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> MDField {
        match pair.clone().into_inner().next() {
            Some(inner_pair) => {
                match inner_pair.as_rule() {
                    Rule::NullConst => MDField::Null,
                    Rule::Metadata => MDField::Metadata {
                        data: Metadata::build_from(&inner_pair)
                    },
                    _ => MDField::None
                }
            },
            None => MDField::None
        }
    }
}