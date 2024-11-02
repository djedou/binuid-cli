use super::{Constant, Ident, InlineAsm};


#[derive(Debug)]
pub enum Value { 
    Constant {
        const_: Constant
    },
	LocalIdent {
        ident: Ident
    },
	InlineAsm {
        asm: InlineAsm
    },
    Poison
}