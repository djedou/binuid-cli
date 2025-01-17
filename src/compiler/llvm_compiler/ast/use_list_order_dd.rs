use crate::compiler::{llvm_compiler::Rule, BuildFrom};
use binuid_shared_wasm::ast_bits::composes::{UseListOrderBB, Ident, Index};



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