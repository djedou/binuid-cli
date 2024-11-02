use super::{Type, Ident};


#[derive(Debug)]
pub struct Case { 
    pub type_: Type,
    pub int_const: u32,
    pub local_ident: Ident,
}