use super::{Value, Ident};
use crate::compiler::{llvm_compiler::Rule, BuildFrom};

#[derive(Debug)]
pub struct Inc { 
    pub value: Value,
    pub local_ident: Ident,
}


impl Inc {
    pub fn new() -> Inc {
        Inc {
            value: Value::None,
            local_ident: Ident::None
        }
    }
}


impl BuildFrom for Inc {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> Inc {
        let mut item = Inc::new();

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::Value => {
                    item.value = Value::build_from(&inner_pair);
                },
                Rule::Type => {
                    item.local_ident = Ident::build_from(&inner_pair);
                },
                _ => {}
            }
        }

        item
    }
}