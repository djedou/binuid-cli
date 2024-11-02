use super::Ident;


#[derive(Debug)]
pub struct BlockAddressConst {
    pub global_ident: Ident,
    pub local_ident: Ident
}