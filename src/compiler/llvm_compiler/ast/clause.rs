use super::{Type, Value, ArrayConst};
use crate::compiler::{llvm_compiler::Rule, BuildFrom};



#[derive(Debug)]
pub enum Clause {
    None,
    Catch {
        catch: ClauseCatch,
    },
	Filter {
        filter: ClauseFilter,
    }
}

#[derive(Debug)]
pub struct ClauseCatch {
    type_: Type,
    value: Value
}

#[derive(Debug)]
pub struct ClauseFilter {
    type_: Type,
    array_const: ArrayConst
}

impl ClauseCatch {
    pub fn new() -> ClauseCatch {
        ClauseCatch {
            type_: Type::None,
            value: Value::None
        }
    }
}

impl ClauseFilter {
    pub fn new() -> ClauseFilter {
        ClauseFilter {
            type_: Type::None,
            array_const: ArrayConst::new()
        }
    }
}



impl BuildFrom for ClauseCatch {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> ClauseCatch {
        let mut item = ClauseCatch::new();

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::Type => {
                    item.type_ = Type::build_from(&inner_pair);
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



impl BuildFrom for ClauseFilter {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> ClauseFilter {
        let mut item = ClauseFilter::new();

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::Type => {
                    item.type_ = Type::build_from(&inner_pair);
                },
                Rule::ArrayConst => {
                    item.array_const = ArrayConst::build_from(&inner_pair);
                },
                _ => {}
            }
        }

        item
    }
}




impl BuildFrom for Clause {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> Clause {
        match pair.clone().into_inner().next() {
            Some(inner_pair) => {
                match inner_pair.as_rule() {
                    Rule::clauseCatch => Clause::Catch {
                        catch: ClauseCatch::build_from(&inner_pair),
                    },
                    Rule::ClauseFilter => Clause::Filter {
                        filter: ClauseFilter::build_from(&inner_pair),
                    },
                    _ => Clause::None
                }
            },
            None => Clause::None
        }
    }
}