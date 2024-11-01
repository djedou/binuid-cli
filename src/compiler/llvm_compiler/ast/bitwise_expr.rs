use super::{OpFlag, TypeConst};

pub enum BitwiseOp {
    Shl,
    LShr,
    AShr,
    And,
    Or,
    Xor
}

pub struct BitwiseExpr { 
    pub op: BitwiseOp,
    pub op_flag: Option<OpFlag>,
    pub lhs: Box<TypeConst>,
    pub rhs: Box<TypeConst>
}