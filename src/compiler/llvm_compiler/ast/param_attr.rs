use super::{Alignment, Dereferenceable, Sret, Range};


pub enum ParamAttr { 
    Alignment {
        align: Alignment
    },
	Dereferenceable {
        der: Dereferenceable
    },
	String {
        value: String
    },
	Byval,
	Inalloca,
	Inreg,
	Nest,
	Noalias,
	Nocapture,
	Nonnull,
	Readnone,
	Readonly,
	Returned,
	Signext,
	Sret {
        sret: Sret
    },
	Swifterror,
	Swiftself,
	Writeonly,
	Zeroext,
    Noundef,
    DeadOnUnwind,
    Writable,
    Immarg,
    Allocalign,
    Allocptr,
    Range {
        range: Range
    }
}