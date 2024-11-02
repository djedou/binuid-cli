use super::TypeValue;


#[derive(Debug)]
pub struct OperandBundle { 
    pub name: String,
    pub type_values: Vec<TypeValue>
}