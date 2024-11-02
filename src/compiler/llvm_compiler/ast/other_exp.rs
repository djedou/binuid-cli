use super::{IPred, FPred, TypeConst};

#[derive(Debug)]
pub enum PredFlag {
    IPred {
        pred: IPred
    },
    FPred {
        pred: FPred
    }
}

#[derive(Debug)]
pub enum OtherOp {
    ICmp,
    FCmp,
    Select
}

#[derive(Debug)]
pub struct OtherExp {
    pub op: OtherOp,
    pub flag: Option<PredFlag>,
    pub lhs: TypeConst,
    pub rhs: TypeConst,
    pub type_const: Option<TypeConst>
}