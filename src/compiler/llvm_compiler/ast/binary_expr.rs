use super::{OpFlag, TypeConst};
use crate::compiler::{llvm_compiler::Rule, BuildFrom};

#[derive(Debug)]
pub enum BinaryOp {
    None,
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
    pub op_flag: OpFlag,
    pub lhs: Box<TypeConst>,
    pub rhs: Box<TypeConst>
}


impl BuildFrom for BinaryExpr {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> BinaryExpr {
        let mut binary_expr = BinaryExpr { 
            op: BinaryOp::None,
            op_flag: OpFlag::None,
            lhs: Box::new(TypeConst::new()),
            rhs: Box::new(TypeConst::new())
        };

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::BinaryOp => {
                    binary_expr.op = BinaryOp::build_from(&inner_pair);
                },
                Rule::OpFlag => {
                    binary_expr.op_flag = OpFlag::build_from(&inner_pair);
                },
                Rule::LhsExpr => {
                    match inner_pair.clone().into_inner().next() {
                        Some(p) => {
                            match p.as_rule() {
                                Rule::TypeConst => {
                                    binary_expr.lhs = Box::new(TypeConst::build_from(&inner_pair));
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
                                    binary_expr.rhs = Box::new(TypeConst::build_from(&inner_pair));
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
        
        binary_expr
    }
}

impl BuildFrom for BinaryOp {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> BinaryOp {
        match pair.clone().into_inner().next() {
            Some(inner_pair) => {
                match inner_pair.as_rule() {
                    Rule::Add => BinaryOp::Add,
                    Rule::FAdd => BinaryOp::FAdd,
                    Rule::Sub => BinaryOp::Sub,
                    Rule::FSub => BinaryOp::FSub,
                    Rule::Mul => BinaryOp::Mul,
                    Rule::FMul => BinaryOp::FMul,
                    Rule::UDiv => BinaryOp::UDiv,
                    Rule::SDiv => BinaryOp::SDiv,
                    Rule::FDiv => BinaryOp::FDiv,
                    Rule::URem => BinaryOp::URem,
                    Rule::SRem => BinaryOp::SRem,
                    Rule::FRem => BinaryOp::FRem,
                    _ => BinaryOp::None
                }
            },
            None => BinaryOp::None
        }
    }
}