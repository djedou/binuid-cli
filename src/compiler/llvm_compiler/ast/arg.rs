use super::{ConcreteType, ParamAttr, Metadata, Value};

pub enum Arg {
    Concret {
        type_: ConcreteType,
        param_attrs: Vec<ParamAttr>,
        value: Value
    },
    Metadata {
        meta: Metadata
    }
}