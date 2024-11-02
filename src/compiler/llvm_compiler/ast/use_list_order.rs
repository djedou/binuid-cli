use super::{Type, Value, Index};


#[derive(Debug)]
pub struct UseListOrder { 
    pub type_: Type,
    pub value: Value,
    pub index: Vec<Index> 
}
