use super::TypeConst;


#[derive(Debug)]
pub enum MemoryOp {
    GetElementPtr
}


#[derive(Debug)]
pub struct MemoryExpr { 
    pub op: MemoryOp,
    pub in_bounds: bool,
    pub lhs: TypeConst,
    pub rhs: TypeConst,
    pub gep_const_indices: Vec<GEPConstIndex>
}


#[derive(Debug)]
pub struct GEPConstIndex { 
    pub inrange: bool,
    pub type_const: TypeConst
}
