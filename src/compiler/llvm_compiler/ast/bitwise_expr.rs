use crate::compiler::{llvm_compiler::Rule, BuildFrom};
use binuid_shared_wasm::ast_bits::ops::{BitwiseOp, OpFlag};
use binuid_shared_wasm::ast_bits::exprs::BitwiseExpr;
use binuid_shared_wasm::ast_bits::types::TypeConst;


impl BuildFrom for BitwiseExpr {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> BitwiseExpr {
        let mut bitwise_expr = BitwiseExpr { 
            op: BitwiseOp::None,
            op_flag: OpFlag::None,
            lhs: Box::new(TypeConst::new()),
            rhs: Box::new(TypeConst::new())
        };

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::BitwiseOp => {
                    bitwise_expr.op = BitwiseOp::build_from(&inner_pair);
                },
                Rule::OpFlag => {
                    bitwise_expr.op_flag = OpFlag::build_from(&inner_pair);
                },
                Rule::LhsExpr => {
                    match inner_pair.clone().into_inner().next() {
                        Some(p) => {
                            match p.as_rule() {
                                Rule::TypeConst => {
                                    bitwise_expr.lhs = Box::new(TypeConst::build_from(&inner_pair));
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
                                    bitwise_expr.rhs = Box::new(TypeConst::build_from(&inner_pair));
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
        
        bitwise_expr
    }
}

impl BuildFrom for BitwiseOp {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> BitwiseOp {
        match pair.clone().into_inner().next() {
            Some(inner_pair) => {
                match inner_pair.as_rule() {
                    Rule::Shl => BitwiseOp::Shl,
                    Rule::LShr => BitwiseOp::LShr,
                    Rule::AShr => BitwiseOp::AShr,
                    Rule::And => BitwiseOp::And,
                    Rule::Or => BitwiseOp::Or,
                    Rule::Xor => BitwiseOp::Xor,
                    _ => BitwiseOp::None
                }
            },
            None => BitwiseOp::None
        }
    }
}