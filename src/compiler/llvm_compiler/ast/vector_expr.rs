use crate::compiler::{llvm_compiler::Rule, BuildFrom};
use binuid_shared_wasm::ast_bits::ops::VectorOp;
use binuid_shared_wasm::ast_bits::exprs::VectorExpr;
use binuid_shared_wasm::ast_bits::types::TypeConst;



impl BuildFrom for VectorExpr {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> VectorExpr {
        let mut vector_expr = VectorExpr { 
            op: VectorOp::None,
            type_: TypeConst::new(),
            lhs: TypeConst::new(),
            rhs: TypeConst::new()
        };

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::VectorOp => {
                    vector_expr.op = VectorOp::build_from(&inner_pair);
                },
                Rule::TypeConst => {
                    vector_expr.type_ = TypeConst::build_from(&inner_pair);
                },
                Rule::LhsExpr => {
                    match inner_pair.clone().into_inner().next() {
                        Some(p) => {
                            match p.as_rule() {
                                Rule::TypeConst => {
                                    vector_expr.lhs = TypeConst::build_from(&inner_pair);
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
                                    vector_expr.rhs = TypeConst::build_from(&inner_pair);
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
        
        vector_expr
    }
}

impl BuildFrom for VectorOp {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> VectorOp {
        match pair.clone().into_inner().next() {
            Some(inner_pair) => {
                match inner_pair.as_rule() {
                    Rule::ExtractElement => VectorOp::ExtractElement,
                    Rule::InsertElement => VectorOp::InsertElement,
                    Rule::ShuffleVector => VectorOp::ShuffleVector,
                    _ => VectorOp::None
                }
            },
            None => VectorOp::None
        }
    }
}