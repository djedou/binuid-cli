use super::{Type, Ident};

pub enum TypeKind {
    Opaque,
    Type {
        type_: Type
    }
}

pub struct TypeDef {
    pub local_ident: Ident,
    pub kind: TypeKind
}