use super::TypeConst;

pub enum VectorOp {
    ExtractElement,
    InsertElement,
    ShuffleVector
}
pub struct VectorExpr {
    pub op: VectorOp,
    pub lhs: TypeConst,
    pub rhs: TypeConst,
    pub type_: Option<TypeConst>
}
