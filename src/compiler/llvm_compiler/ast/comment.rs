use crate::compiler::{llvm_compiler::Rule, BuildFrom};
use binuid_shared_wasm::ast_bits::composes::Comment;


impl BuildFrom for Comment {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> Comment {
        match pair.clone().into_inner().next() {
            Some(p) => {
                Comment {
                    value: String::build_from(&p)
                }
            },
            None => {
                Comment {
                    value: String::with_capacity(0)
                }
            }
        }
    }
}