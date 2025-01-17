use crate::compiler::{llvm_compiler::Rule, BuildFrom};
use binuid_shared_wasm::ast_bits::values::Value;
use binuid_shared_wasm::ast_bits::types::Type;
use binuid_shared_wasm::ast_bits::types::TypeValue;



impl BuildFrom for TypeValue {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> TypeValue {
        let mut type_value = TypeValue::new();

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::Type => {
                    type_value.type_ = Type::build_from(&inner_pair);
                },
                Rule::Value => {
                    type_value.value = Value::build_from(&inner_pair);
                },
                _ => {}
            }
        }

        type_value
    }
}