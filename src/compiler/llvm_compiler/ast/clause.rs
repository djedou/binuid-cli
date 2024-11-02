use super::{Type, Value, ArrayConst};


#[derive(Debug)]
pub enum Clause { 
    Catch {
        type_: Type,
        value: Value
    },
	Filter {
        type_: Type,
        array_const: ArrayConst
    }
}