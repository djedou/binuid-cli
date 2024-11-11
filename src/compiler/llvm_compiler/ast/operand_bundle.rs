use super::TypeValue;
use crate::compiler::{llvm_compiler::Rule, BuildFrom};


#[derive(Debug)]
pub struct OperandBundle { 
    pub name: String,
    pub type_values: Vec<TypeValue>
}


impl OperandBundle {
    pub fn new() -> OperandBundle {
        OperandBundle {
            name: String::with_capacity(0),
            type_values: vec![]
        }
    }
}


impl BuildFrom for OperandBundle {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> OperandBundle {
        let mut item = OperandBundle::new();

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::StringLit => {
                    item.name = String::build_from(&inner_pair);
                },
                Rule::TypeValues => {
                    for p in inner_pair.clone().into_inner() {
                        match p.as_rule() {
                            Rule::TypeValue => {
                                item.type_values.push(TypeValue::build_from(&p));
                            },
                            _ => {}
                        }
                    }
                },
                _ => {}
            }
        }

        item
    }
}