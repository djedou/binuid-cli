use super::{Type, TypeConst};


#[derive(Debug)]
pub enum ConversionOp  {
    Trunc,
    ZExt,
    SExt,
    FPTrunc,
    FPExt,
    FPToUI,
    FPToSI,
    UIToFP,
    SIToFP,
    PtrToInt,
    IntToPtr,
    BitCast,
    AddrSpaceCast
}

#[derive(Debug)]
pub struct ConversionExpr {
    pub op: ConversionOp,
    pub lhs: TypeConst, 
    pub type_: Type
}
