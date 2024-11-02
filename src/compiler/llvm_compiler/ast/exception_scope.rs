use super::Ident;


#[derive(Debug)]
pub enum ExceptionScope {
    None,
    Local {
        ident: Ident
    }
}