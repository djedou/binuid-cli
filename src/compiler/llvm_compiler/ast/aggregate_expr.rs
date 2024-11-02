use super::{TypeConst, Index};

#[derive(Debug)]
pub enum AggregateOp {
    ExtractValue,
    InsertValue
}

#[derive(Debug)]
pub struct AggregateExpr {
    pub op: AggregateOp,
    pub lhs: TypeConst,
    pub rhs: TypeConst,
    pub indices: Vec<Index>
}
