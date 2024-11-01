use super::{TypeConst, Index};

pub enum AggregateOp {
    ExtractValue,
    InsertValue
}
pub struct AggregateExpr {
    pub op: AggregateOp,
    pub lhs: TypeConst,
    pub rhs: TypeConst,
    pub indices: Vec<Index>
}
