use super::Ident;
use crate::compiler::{llvm_compiler::Rule, BuildFrom};


#[derive(Debug)]
pub enum UnwindTarget {
    None,
    ToCaller,
	LocalLabel {
        label: Ident
    }
}


impl BuildFrom for UnwindTarget {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> UnwindTarget {
        match pair.clone().into_inner().next() {
            Some(inner_pair) => {
                match inner_pair.as_rule() {
                    Rule::ToCaller => UnwindTarget::ToCaller,
                    Rule::LocalLabel => UnwindTarget::LocalLabel {
                        label: Ident::build_from(&inner_pair)
                    },
                    _ => UnwindTarget::None
                }
            },
            None => UnwindTarget::None
        }
    }
}