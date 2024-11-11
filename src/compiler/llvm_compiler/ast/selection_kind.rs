use crate::compiler::{llvm_compiler::Rule, BuildFrom};

#[derive(Debug)]
pub enum SelectionKind {
    None,
    Any,
	ExactMatch,
	Largest,
	Noduplicates,
	Samesize,
}


impl BuildFrom for SelectionKind {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> SelectionKind {
        match pair.clone().into_inner().next() {
            Some(inner_pair) => {
                match inner_pair.as_rule() {
                    Rule::Any => SelectionKind::Any,
                    Rule::ExactMatch => SelectionKind::ExactMatch,
                    Rule::Largest => SelectionKind::Largest,
                    Rule::Noduplicates => SelectionKind::Noduplicates,
                    Rule::Samesize => SelectionKind::Samesize,
                    _ => SelectionKind::None
                }
            },
            None => SelectionKind::None
        }
    }
}