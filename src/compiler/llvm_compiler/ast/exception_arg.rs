use super::{ConcreteType, Value, Metadata};

pub enum ExceptionArg { 
    Concrete {
        type_: ConcreteType,
        value: Value
    },
    Metadata {
        metadata: Metadata
    }
}