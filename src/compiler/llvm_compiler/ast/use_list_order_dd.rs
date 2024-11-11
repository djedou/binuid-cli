use super::{Ident, Index};
use crate::compiler::{llvm_compiler::Rule, BuildFrom};


#[derive(Debug)]
pub struct UseListOrderBB { 
    pub global_ident: Ident,
    pub local_ident: Ident,
    pub indexes: Vec<Index> 
}


impl UseListOrderBB {
    pub fn new() -> UseListOrderBB {
        UseListOrderBB { 
            global_ident: Ident::None,
            local_ident: Ident::None,
            indexes: vec![]
        } 
    }
}


impl BuildFrom for UseListOrderBB {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> UseListOrderBB {
        let mut list_order = UseListOrderBB::new();

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::GlobalIdent => {
                    list_order.global_ident = Ident::build_from(&inner_pair);
                },
                Rule::LocalIdent => {
                    list_order.local_ident = Ident::build_from(&inner_pair);
                },
                Rule::Indices => {
                    for p in inner_pair.clone().into_inner() {
                        match p.as_rule() {
                            Rule::Index => {
                                list_order.indexes.push(Index::build_from(&p));
                            },
                            _ => {}
                        }
                    }
                },
                _ => {}
            }
        }

        list_order
    }
}