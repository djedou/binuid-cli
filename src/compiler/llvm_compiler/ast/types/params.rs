

pub enum Params {
    ListWithDots {
        params: Vec<Param>   
    },
    List {
        params: Vec<Param>
    },
    DotDotDot
}

pub enum Param {
    WithIdent {
        type_: Type,
        attrs: Vec<ParamAttr>,
        ident: String
    },
    NoIdent {
        type_: Type,
        attrs: Vec<ParamAttr>
    }
}


pub enum ParamAttr { 
    Alignment,
	Dereferenceable,
	StringLit,
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
	Sret,
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
    Range
}