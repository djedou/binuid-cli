use super::{Type, Ident};


pub struct Case { 
    pub type_: Type,
    pub int_const: u32,
    pub local_ident: Ident,
}