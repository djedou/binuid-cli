use super::{ConcreteType, Value, Metadata};
use crate::compiler::{llvm_compiler::Rule, BuildFrom};


#[derive(Debug)]
pub enum ExceptionArg {
    None,
    Concrete {
        concrete: ExceptionArgConcret
    },
    Metadata {
        meta: ExceptionArgMetadata
    }
}



#[derive(Debug)]
pub struct ExceptionArgConcret {
    type_: ConcreteType,
    value: Value
}

#[derive(Debug)]
pub struct ExceptionArgMetadata {
    meta: Metadata
}

impl ExceptionArgMetadata {
    pub fn new() -> ExceptionArgMetadata {
        ExceptionArgMetadata {
            meta: Metadata::None
        }
    }
}


impl ExceptionArgConcret {
    pub fn new() -> ExceptionArgConcret {
        ExceptionArgConcret {
            type_: ConcreteType::None,
            value: Value::None
        }
    }
}


impl BuildFrom for ExceptionArgMetadata {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> ExceptionArgMetadata {
        let mut item = ExceptionArgMetadata::new();

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::Metadata => {
                    item.meta = Metadata::build_from(&inner_pair);
                },
                _ => {}
            }
        }

        item
    }
}


impl BuildFrom for ExceptionArgConcret {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> ExceptionArgConcret {
        let mut item = ExceptionArgConcret::new();

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::ConcreteType => {
                    item.type_ = ConcreteType::build_from(&inner_pair);
                },
                Rule::Value => {
                    item.value = Value::build_from(&inner_pair);
                },
                _ => {}
            }
        }

        item
    }
}



impl BuildFrom for ExceptionArg {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> ExceptionArg {
        match pair.clone().into_inner().next() {
            Some(inner_pair) => {
                match inner_pair.as_rule() {
                    Rule::ExceptionArgConcrete => ExceptionArg::Concrete {
                        concrete: ExceptionArgConcret::build_from(&inner_pair)
                    },
                    Rule::ExceptionArgMetadata => ExceptionArg::Metadata {
                        meta: ExceptionArgMetadata::build_from(&inner_pair)
                    },
                    _ => ExceptionArg::None
                }
            },
            None => ExceptionArg::None
        }
    }
}