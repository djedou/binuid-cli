
use super::{Alignment, Dereferenceable, Range};


#[derive(Debug)]
pub enum ReturnAttr { 
    Align {
        align: Alignment
    },
	Dereferenceable {
        deref: Dereferenceable
    },
	String {
        data: String
    },
	Inreg,
	Noalias,
	Nonnull,
	Signext,
	Zeroext,
    Noundef,
    Range {
        range: Range
    },
}