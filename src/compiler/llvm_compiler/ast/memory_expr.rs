use super::TypeConst;
use crate::compiler::{llvm_compiler::Rule, BuildFrom};


#[derive(Debug)]
pub enum MemoryOp {
    None,
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


impl BuildFrom for MemoryExpr {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> MemoryExpr {
        let mut memory_expr = MemoryExpr { 
            op: MemoryOp::None,
            in_bounds: false,
            lhs: TypeConst::new(),
            rhs: TypeConst::new(),
            gep_const_indices: vec![]
        };

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::MemoryOp => {
                    memory_expr.op = MemoryOp::build_from(&inner_pair);
                },
                Rule::InBounds => {
                    memory_expr.in_bounds = true;
                },
                Rule::GEPConstIndices => {
                    let mut indices = vec![];
                    for p in inner_pair.clone().into_inner() {
                        match p.as_rule() {
                            Rule::GEPConstIndex => {
                                indices.push(GEPConstIndex::build_from(&p));
                            },
                            _ => {}
                        }
                    }
                    memory_expr.gep_const_indices = indices;
                },
                Rule::LhsExpr => {
                    match inner_pair.clone().into_inner().next() {
                        Some(p) => {
                            match p.as_rule() {
                                Rule::TypeConst => {
                                    memory_expr.lhs = TypeConst::build_from(&inner_pair);
                                },
                                _ => {}
                            }
                        },
                        None => {}
                    }
                },
                Rule::RhsExpr => {
                    match inner_pair.clone().into_inner().next() {
                        Some(p) => {
                            match p.as_rule() {
                                Rule::TypeConst => {
                                    memory_expr.rhs = TypeConst::build_from(&inner_pair);
                                },
                                _ => {}
                            }
                        },
                        None => {}
                    }
                },
                _ => {}
            }
        }
        
        memory_expr
    }
}

impl BuildFrom for GEPConstIndex {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> GEPConstIndex {
        let mut gep_const_index = GEPConstIndex { 
            inrange: false,
            type_const: TypeConst::new()
        };

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::Inrange => {
                    gep_const_index.inrange = true;
                },
                Rule::TypeConst => {
                    gep_const_index.type_const = TypeConst::build_from(&inner_pair);
                },
                _ => {}
            }
        }
        
        gep_const_index
    }
}

impl BuildFrom for MemoryOp {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> MemoryOp {
        match pair.clone().into_inner().next() {
            Some(inner_pair) => {
                match inner_pair.as_rule() {
                    Rule::GetElementPtr => MemoryOp::GetElementPtr,
                    _ => MemoryOp::None
                }
            },
            None => MemoryOp::None
        }
    }
}