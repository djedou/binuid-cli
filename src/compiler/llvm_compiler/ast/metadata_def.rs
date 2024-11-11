use super::{MetadataId, MDTuple};
use crate::compiler::{llvm_compiler::Rule, BuildFrom};



#[derive(Debug)]
pub struct MetadataDef {
    pub id: MetadataId,
    pub distinct: bool,
    pub md_tuple: MDTuple
}


impl MetadataDef {
    pub fn new() -> MetadataDef {
        MetadataDef {
            id: MetadataId::new(),
            distinct: false,
            md_tuple: MDTuple::new()
        }
    }
}


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