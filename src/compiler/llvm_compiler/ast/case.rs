use super::{Type, Ident};
use crate::compiler::{llvm_compiler::Rule, BuildFrom};


#[derive(Debug)]
pub struct Case { 
    pub type_: Type,
    pub int_const: u32,
    pub local_ident: Ident,
}


impl Case {
    pub fn new() -> Case {
        Case {
            type_: Type::None,
            int_const: 0,
            local_ident: Ident::None
        }
    }
}

impl BuildFrom for Case {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> Case {
        let mut item = Case::new();

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::Type => {
                    item.type_ = Type::build_from(&inner_pair);
                },
                Rule::IntConst => {
                    item.int_const = inner_pair.as_str().parse::<u32>().map_or(0, |d| d.clone());
                },
                Rule::LocalIdent => {
                    item.local_ident = Ident::build_from(&inner_pair);
                },
                _ => {}
            }
        }

        item
    }
}