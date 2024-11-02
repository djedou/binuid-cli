use super::{ConcreteType, Value, Metadata};


#[derive(Debug)]
pub enum ExceptionArg { 
    Concrete {
        type_: ConcreteType,
        value: Value
    },
    Metadata {
        metadata: Metadata
    }
}