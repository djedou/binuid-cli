use super::{Type, Value};
use crate::compiler::{llvm_compiler::Rule, BuildFrom};

#[derive(Debug)]
pub struct TypeValue {
    pub type_: Type,
    pub value: Value
}


impl TypeValue {
    pub fn new() -> TypeValue {
        TypeValue {
            type_: Type::None,
            value: Value::None,
        }
    }
}


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