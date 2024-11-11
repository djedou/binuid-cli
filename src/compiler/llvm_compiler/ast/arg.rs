use super::{ConcreteType, ParamAttr, Metadata, Value};
use crate::compiler::{llvm_compiler::Rule, BuildFrom};


#[derive(Debug)]
pub enum Arg {
    None,
    Concret {
        concret: ArgConcret
    },
    Metadata {
        meta: ArgMetadata
    }
}

#[derive(Debug)]
pub struct ArgConcret {
    type_: ConcreteType,
    param_attrs: Vec<ParamAttr>,
    value: Value
}

#[derive(Debug)]
pub struct ArgMetadata {
    meta: Metadata
}

impl ArgMetadata {
    pub fn new() -> ArgMetadata {
        ArgMetadata {
            meta: Metadata::None
        }
    }
}


impl ArgConcret {
    pub fn new() -> ArgConcret {
        ArgConcret {
            type_: ConcreteType::None,
            param_attrs: vec![],
            value: Value::None
        }
    }
}


impl BuildFrom for ArgMetadata {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> ArgMetadata {
        let mut item = ArgMetadata::new();

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


impl BuildFrom for ArgConcret {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> ArgConcret {
        let mut item = ArgConcret::new();

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::ConcreteType => {
                    item.type_ = ConcreteType::build_from(&inner_pair);
                },
                Rule::ParamAttrs => {
                    for p in inner_pair.clone().into_inner() {
                        match p.as_rule() {
                            Rule::ParamAttr => {
                                item.param_attrs.push(ParamAttr::build_from(&p));
                            },
                            _ => {}
                        }
                    }
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



impl BuildFrom for Arg {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> Arg {
        match pair.clone().into_inner().next() {
            Some(inner_pair) => {
                match inner_pair.as_rule() {
                    Rule::ArgConcret => Arg::Concret {
                        concret: ArgConcret::build_from(&inner_pair)
                    },
                    Rule::ArgMetadata => Arg::Metadata {
                        meta: ArgMetadata::build_from(&inner_pair)
                    },
                    _ => Arg::None
                }
            },
            None => Arg::None
        }
    }
}