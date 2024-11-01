use super::TypeConst;

pub enum MemoryOp {
    GetElementPtr
}

pub struct MemoryExpr { 
    pub op: MemoryOp,
    pub in_bounds: bool,
    pub lhs: TypeConst,
    pub rhs: TypeConst,
    pub gep_const_indices: Vec<GEPConstIndex>
}

pub struct GEPConstIndex { 
    pub inrange: bool,
    pub type_const: TypeConst
}
