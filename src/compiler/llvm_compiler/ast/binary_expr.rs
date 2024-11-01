use super::{OpFlag, TypeConst};

pub enum BinaryOp {
    Add,
    FAdd,
    Sub,
    FSub,
    Mul,
    FMul,
    UDiv,
    SDiv,
    FDiv,
    URem,
    SRem,
    FRem
}
pub struct BinaryExpr { 
    pub op: BinaryOp,
    pub op_flag: Option<OpFlag>,
    pub lhs: Box<TypeConst>,
    pub rhs: Box<TypeConst>
}
