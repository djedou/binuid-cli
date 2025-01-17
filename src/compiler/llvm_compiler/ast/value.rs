use crate::compiler::{llvm_compiler::Rule, BuildFrom};
use binuid_shared_wasm::ast_bits::values::Value;
use binuid_shared_wasm::ast_bits::composes::InlineAsm;
use binuid_shared_wasm::ast_bits::composes::Ident;
use binuid_shared_wasm::ast_bits::values::Constant;



impl BuildFrom for Value {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> Value {
        match pair.clone().into_inner().next() {
            Some(inner_pair) => {
                match inner_pair.as_rule() {
                    Rule::Constant => Value::Constant {
                        const_: Constant::build_from(&inner_pair)
                    },
                    Rule::LocalIdent => Value::LocalIdent {
                        ident: Ident::build_from(&inner_pair)
                    },
                    Rule::InlineAsm => Value::InlineAsm {
                        asm: InlineAsm::build_from(&inner_pair)
                    },
                    Rule::Poison => Value::Poison,
                    _ => Value::None
                }
            },
            None => Value::None
        }
    }
}