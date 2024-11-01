use super::Ident;

pub enum ExceptionScope {
    None,
    Local {
        ident: Ident
    }
}