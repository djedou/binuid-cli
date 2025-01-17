use binuid_shared_wasm::ast_bits::composes::Ident;
use binuid_shared_wasm::ast_bits::types::Type;
use crate::compiler::{llvm_compiler::Rule, BuildFrom};
use binuid_shared_wasm::ast_bits::composes::Case;

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