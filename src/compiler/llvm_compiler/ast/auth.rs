use crate::compiler::{llvm_compiler::Rule, BuildFrom};


#[derive(Debug)]
pub enum Auth {
    None,
    Readwrite,
    Writeonly,
    Readonly,
    Read,
    Write,
}

impl BuildFrom for Auth {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> Auth {
        match pair.clone().into_inner().next() {
            Some(inner_pair) => {
                match inner_pair.as_rule() {
                    Rule::Readwrite => Auth::Readwrite,
                    Rule::Writeonly => Auth::Writeonly,
                    Rule::Readonly => Auth::Readonly,
                    Rule::Read => Auth::Read,
                    Rule::Write => Auth::Write,
                    _ => Auth::None
                }
            },
            None => Auth::None
        }
    }
}