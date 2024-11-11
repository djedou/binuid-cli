use super::TLSModel;
use crate::compiler::{llvm_compiler::Rule, BuildFrom};

#[derive(Debug)]
pub struct ThreadLocal {
    pub model: TLSModel
}

impl ThreadLocal {
    pub fn new() -> ThreadLocal {
        ThreadLocal {
            model: TLSModel::None
        }
    }
}

impl BuildFrom for ThreadLocal {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> ThreadLocal {
        let mut thread_local = ThreadLocal {
            model: TLSModel::None
        };

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::TLSModel => {
                    thread_local.model = TLSModel::build_from(&inner_pair);
                },
                _ => {}
            }
        }

        thread_local
    }
}