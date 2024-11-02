use super::TypeConst;


#[derive(Debug)]
pub enum VectorOp {
    ExtractElement,
    InsertElement,
    ShuffleVector
}

#[derive(Debug)]
pub struct VectorExpr {
    pub op: VectorOp,
    pub lhs: TypeConst,
    pub rhs: TypeConst,
    pub type_: Option<TypeConst>
}
