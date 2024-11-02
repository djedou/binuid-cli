use super::{Value, Ident};


#[derive(Debug)]
pub struct Inc { 
    pub value: Value,
    pub local_ident: Ident,
}