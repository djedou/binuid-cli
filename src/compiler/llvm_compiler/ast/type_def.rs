use super::{Type, Ident};

#[derive(Debug)]
pub enum TypeKind {
    Opaque,
    Type {
        type_: Type
    }
}

#[derive(Debug)]
pub struct TypeDef {
    pub local_ident: Ident,
    pub kind: TypeKind
}