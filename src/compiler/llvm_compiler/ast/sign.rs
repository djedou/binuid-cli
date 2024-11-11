use crate::compiler::{llvm_compiler::Rule, BuildFrom};

#[derive(Debug)]
pub enum Sign {
    None,
    Plus,
	Minus
}

impl BuildFrom for Sign {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> Sign {
        match pair.clone().into_inner().next() {
            Some(inner_pair) => {
                match inner_pair.as_rule() {
                    Rule::SignPlus => Sign::Plus,
                    Rule::SignMinus => Sign::Minus,
                    _ => Sign::None
                }
            },
            None => Sign::None
        }
    }
}