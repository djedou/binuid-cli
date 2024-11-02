use super::{ParamAttr, Ident, Type};

#[derive(Debug)]
pub struct Param { 
    pub type_: Type,
    pub param_attrs: Vec<ParamAttr>,
    pub local_ident: Option<Ident>
}