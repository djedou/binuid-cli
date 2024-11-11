
use crate::compiler::{llvm_compiler::Rule, BuildFrom};

#[derive(Debug)]
pub enum OverflowFlag {
    None,
    Nsw,
	Nuw 
}


impl BuildFrom for OverflowFlag {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> OverflowFlag {
        match pair.clone().into_inner().next() {
            Some(inner_pair) => {
                match inner_pair.as_rule() {
                    Rule::Nsw => OverflowFlag::Nsw,
                    Rule::Nuw => OverflowFlag::Nuw,
                    _ => OverflowFlag::None
                }
            },
            None => OverflowFlag::None
        }
    }
}