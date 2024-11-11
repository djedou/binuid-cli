use crate::compiler::{llvm_compiler::Rule, BuildFrom};

#[derive(Debug)]
pub enum Alias {
    None,
    Alias,
	Ifunc
}


impl BuildFrom for Alias {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> Alias {
        match pair.clone().into_inner().next() {
            Some(inner_pair) => {
                match inner_pair.as_rule() {
                    Rule::Als => Alias::Alias,
                    Rule::Ifunc => Alias::Ifunc,
                    _ => Alias::None
                }
            },
            None => Alias::None
        }
    }
}