use crate::compiler::{llvm_compiler::Rule, BuildFrom};
use binuid_shared_wasm::ast_bits::composes::UseListOrder;
use binuid_shared_wasm::ast_bits::types::Type;
use binuid_shared_wasm::ast_bits::values::Value;
use binuid_shared_wasm::ast_bits::composes::Index;




impl BuildFrom for UseListOrder {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> UseListOrder {
        let mut list_order = UseListOrder::new();

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::Type => {
                    list_order.type_ = Type::build_from(&inner_pair);
                },
                Rule::Type => {
                    list_order.value = Value::build_from(&inner_pair);
                },
                Rule::Indices => {
                    for p in inner_pair.clone().into_inner() {
                        match p.as_rule() {
                            Rule::Index => {
                                list_order.index.push(Index::build_from(&p));
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