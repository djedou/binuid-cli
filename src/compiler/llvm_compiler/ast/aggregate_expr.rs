use super::{TypeConst, Index};
use crate::compiler::{llvm_compiler::Rule, BuildFrom};

#[derive(Debug)]
pub enum AggregateOp {
    None,
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


impl BuildFrom for AggregateExpr {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> AggregateExpr {
        let mut aggregate_expr = AggregateExpr { 
            op: AggregateOp::None,
            indices: vec![],
            lhs: TypeConst::new(),
            rhs: TypeConst::new()
        };

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::AggregateOp => {
                    aggregate_expr.op = AggregateOp::build_from(&inner_pair);
                },
                Rule::Indices => {
                    for p in inner_pair.into_inner() {
                        match p.as_rule() {
                            Rule::Index => {
                                aggregate_expr.indices.push(Index::build_from(&p));
                            },
                            _ => {}
                        }
                    }
                },
                Rule::LhsExpr => {
                    match inner_pair.clone().into_inner().next() {
                        Some(p) => {
                            match p.as_rule() {
                                Rule::TypeConst => {
                                    aggregate_expr.lhs = TypeConst::build_from(&inner_pair);
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
                                    aggregate_expr.rhs = TypeConst::build_from(&inner_pair);
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
        
        aggregate_expr
    }
}

impl BuildFrom for AggregateOp {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> AggregateOp {
        match pair.clone().into_inner().next() {
            Some(inner_pair) => {
                match inner_pair.as_rule() {
                    Rule::ExtractValue => AggregateOp::ExtractValue,
                    Rule::InsertValue => AggregateOp::InsertValue,
                    _ => AggregateOp::None
                }
            },
            None => AggregateOp::None
        }
    }
}