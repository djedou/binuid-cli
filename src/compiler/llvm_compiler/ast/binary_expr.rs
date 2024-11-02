use super::{OpFlag, TypeConst};


#[derive(Debug)]
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

#[derive(Debug)]
pub struct BinaryExpr { 
    pub op: BinaryOp,
    pub op_flag: Option<OpFlag>,
    pub lhs: Box<TypeConst>,
    pub rhs: Box<TypeConst>
}
