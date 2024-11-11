use crate::compiler::{llvm_compiler::Rule, BuildFrom};

#[derive(Debug)]
pub enum Visibility {
    None,
    Default,
	Hidden,
	Protected
}


impl BuildFrom for Visibility {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> Visibility {
        match pair.clone().into_inner().next() {
            Some(inner_pair) => {
                match inner_pair.as_rule() {
                    Rule::Default => Visibility::Default,
                    Rule::Hidden => Visibility::Hidden,
                    Rule::Protected => Visibility::Protected,
                    _ => Visibility::None
                }
            },
            None => Visibility::None
        }
    }
}