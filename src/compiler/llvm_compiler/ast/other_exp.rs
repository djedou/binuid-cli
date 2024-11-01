use super::{IPred, FPred, TypeConst};
/*
pub enum IPred { 
    Eq,
	Ne,
	Sge,
	Sgt,
	Sle,
	Slt,
	Uge,
	Ugt,
	Ule,
	Ult
}

pub enum FPred { 
    False,
	True,
    Oeq,
    Oge,
    Ogt,
    Ole,
    Olt,
    One,
    Ord,
    Ueq,
    Uge,
    Ugt,
    Ule,
    Ult,
    Une,
    Uno
}
*/
pub enum PredFlag {
    IPred {
        pred: IPred
    },
    FPred {
        pred: FPred
    }
}

pub enum OtherOp {
    ICmp,
    FCmp,
    Select
}
pub struct OtherExp {
    pub op: OtherOp,
    pub flag: Option<PredFlag>,
    pub lhs: TypeConst,
    pub rhs: TypeConst,
    pub type_const: Option<TypeConst>
}