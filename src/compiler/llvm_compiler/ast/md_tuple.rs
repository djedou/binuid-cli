use super::MDField;
use crate::compiler::{llvm_compiler::Rule, BuildFrom};

#[derive(Debug)]
pub struct MDTuple {
    pub fields: Vec<MDField>
}


impl MDTuple {
    pub fn new() -> MDTuple {
        MDTuple {
            fields: Vec::with_capacity(0)
        }
    }
}


impl BuildFrom for MDTuple {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> MDTuple {
        let mut metadata = MDTuple::new();

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::MDField => {
                    metadata.fields.push(MDField::build_from(&inner_pair));
                },
                _ => {}
            }
        }

        metadata
    }
}