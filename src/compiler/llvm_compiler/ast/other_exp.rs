use crate::compiler::{llvm_compiler::Rule, BuildFrom};
use binuid_shared_wasm::ast_bits::{
    simples::{FPred, IPred},
    types::TypeConst
};
use binuid_shared_wasm::ast_bits::exprs::OtherExp;
use binuid_shared_wasm::ast_bits::ops::OtherOp;
use binuid_shared_wasm::ast_bits::composes::PredFlag;



impl BuildFrom for OtherExp {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> OtherExp {
        let mut other_expr = OtherExp {
            op: OtherOp::None,
            flag: PredFlag::None,
            lhs: TypeConst::new(),
            rhs: TypeConst::new(),
            type_const: TypeConst::new()
        };

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::OtherOp => {
                    other_expr.op = OtherOp::build_from(&inner_pair);
                },
                Rule::PredFlag => {
                    other_expr.flag = PredFlag::build_from(&inner_pair);
                },
                Rule::TypeConst => {
                    other_expr.type_const = TypeConst::build_from(&inner_pair);
                },
                Rule::LhsExpr => {
                    match inner_pair.clone().into_inner().next() {
                        Some(p) => {
                            match p.as_rule() {
                                Rule::TypeConst => {
                                    other_expr.lhs = TypeConst::build_from(&inner_pair);
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
                                    other_expr.rhs = TypeConst::build_from(&inner_pair);
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
        
        other_expr
    }
}

impl BuildFrom for OtherOp {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> OtherOp {
        match pair.clone().into_inner().next() {
            Some(inner_pair) => {
                match inner_pair.as_rule() {
                    Rule::ICmp => OtherOp::ICmp,
                    Rule::FCmp => OtherOp::FCmp,
                    Rule::Select => OtherOp::Select,
                    _ => OtherOp::None
                }
            },
            None => OtherOp::None
        }
    }
}

impl BuildFrom for PredFlag {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> PredFlag {
        match pair.clone().into_inner().next() {
            Some(inner_pair) => {
                match inner_pair.as_rule() {
                    Rule::IPred => PredFlag::IPred {
                        pred: IPred::build_from(&inner_pair)
                    },
                    Rule::FPred => PredFlag::FPred {
                        pred: FPred::build_from(&inner_pair)
                    },
                    _ => PredFlag::None
                }
            },
            None => PredFlag::None
        }
    }
}