use crate::compiler::{llvm_compiler::Rule, BuildFrom};
use binuid_shared_wasm::ast_bits::values::BlockAddressConst;
use binuid_shared_wasm::ast_bits::composes::Ident;




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