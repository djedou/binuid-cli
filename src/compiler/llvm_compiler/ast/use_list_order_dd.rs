use super::{Ident, Index};


#[derive(Debug)]
pub struct UseListOrderBB { 
    pub global_ident: Ident,
    pub local_ident: Ident,
    pub indexes: Vec<Index> 
}