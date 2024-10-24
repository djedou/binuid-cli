use super::Type;

pub enum TypeDef {
    Simple {
        ident: String,
        type_: Type
    },
    Opaque {
        ident: String
    }
}