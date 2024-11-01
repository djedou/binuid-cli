use super::{Ident, Index};

pub struct UseListOrderBB { 
    pub global_ident: Ident,
    pub local_ident: Ident,
    pub indexes: Vec<Index> 
}