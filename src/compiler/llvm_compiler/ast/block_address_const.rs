use super::Ident;
use crate::compiler::{llvm_compiler::Rule, BuildFrom};

#[derive(Debug)]
pub struct BlockAddressConst {
    pub global_ident: Ident,
    pub local_ident: Ident
}

impl BuildFrom for BlockAddressConst {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> BlockAddressConst {
        let mut address = BlockAddressConst {
            global_ident: Ident::None,
            local_ident: Ident::None
        };

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::GlobalIdent => {
                    address.global_ident = Ident::build_from(&inner_pair);
                },
                Rule::LocalIdent => {
                    address.local_ident = Ident::build_from(&inner_pair);
                },
                _ => {}
            }
        }

        address
    }
}