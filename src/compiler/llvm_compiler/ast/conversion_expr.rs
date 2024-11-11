use super::{Type, TypeConst};
use crate::compiler::{llvm_compiler::Rule, BuildFrom};

#[derive(Debug)]
pub enum ConversionOp  {
    None,
    Trunc,
    ZExt,
    SExt,
    FPTrunc,
    FPExt,
    FPToUI,
    FPToSI,
    UIToFP,
    SIToFP,
    PtrToInt,
    IntToPtr,
    BitCast,
    AddrSpaceCast
}

#[derive(Debug)]
pub struct ConversionExpr {
    pub op: ConversionOp,
    pub lhs: TypeConst, 
    pub type_: Type
}



impl BuildFrom for ConversionExpr {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> ConversionExpr {
        let mut conversion_expr = ConversionExpr { 
            op: ConversionOp::None,
            lhs: TypeConst::new(),
            type_: Type::None
        };

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::ConversionOp => {
                    conversion_expr.op = ConversionOp::build_from(&inner_pair);
                },
                Rule::Type => {
                    conversion_expr.type_ = Type::build_from(&inner_pair);
                },
                Rule::LhsExpr => {
                    match inner_pair.clone().into_inner().next() {
                        Some(p) => {
                            match p.as_rule() {
                                Rule::TypeConst => {
                                    conversion_expr.lhs = TypeConst::build_from(&inner_pair);
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
        
        conversion_expr
    }
}

impl BuildFrom for ConversionOp {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> ConversionOp {
        match pair.clone().into_inner().next() {
            Some(inner_pair) => {
                match inner_pair.as_rule() {
                    Rule::Trunc => ConversionOp::Trunc,
                    Rule::ZExt => ConversionOp::ZExt,
                    Rule::SExt => ConversionOp::SExt,
                    Rule::FPTrunc => ConversionOp::FPTrunc,
                    Rule::FPExt => ConversionOp::FPExt,
                    Rule::FPToUI => ConversionOp::FPToUI,
                    Rule::FPToSI => ConversionOp::FPToSI,
                    Rule::UIToFP => ConversionOp::UIToFP,
                    Rule::SIToFP => ConversionOp::SIToFP,
                    Rule::PtrToInt => ConversionOp::PtrToInt,
                    Rule::IntToPtr => ConversionOp::IntToPtr,
                    Rule::BitCast => ConversionOp::BitCast,
                    Rule::AddrSpaceCast => ConversionOp::AddrSpaceCast,
                    _ => ConversionOp::None
                }
            },
            None => ConversionOp::None
        }
    }
}
    