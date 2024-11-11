use super::{BinaryExpr, BitwiseExpr, VectorExpr, AggregateExpr, MemoryExpr, ConversionExpr, OtherExp};
use crate::compiler::{llvm_compiler::Rule, BuildFrom};

#[derive(Debug)]
pub enum ConstantExpr {
    None,
    Binary {
        expr: BinaryExpr
    },
	Bitwise {
        expr: BitwiseExpr
    },
	Vector {
        expr: VectorExpr
    },
	Aggregate {
        expr: AggregateExpr
    },
	Memory {
        expr: MemoryExpr
    },
	Conversion {
        expr: ConversionExpr
    },
	Other {
        expr: OtherExp
    }
}


impl BuildFrom for ConstantExpr {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> ConstantExpr {
        match pair.clone().into_inner().next() {
            Some(inner_pair) => {
                match inner_pair.as_rule() {
                    Rule::BinaryExpr => ConstantExpr::Binary {
                        expr: BinaryExpr::build_from(&inner_pair)
                    },
                    Rule::BitwiseExpr => ConstantExpr::Bitwise {
                        expr: BitwiseExpr::build_from(&inner_pair)
                    },
                    Rule::VectorExpr => ConstantExpr::Vector {
                        expr: VectorExpr::build_from(&inner_pair)
                    },
                    Rule::AggregateExpr => ConstantExpr::Aggregate {
                        expr: AggregateExpr::build_from(&inner_pair)
                    },
                    Rule::MemoryExpr => ConstantExpr::Memory {
                        expr: MemoryExpr::build_from(&inner_pair)
                    },
                    Rule::ConversionExpr => ConstantExpr::Conversion {
                        expr: ConversionExpr::build_from(&inner_pair)
                    },
                    Rule::OtherExp => ConstantExpr::Other {
                        expr: OtherExp::build_from(&inner_pair)
                    },
                    _ => ConstantExpr::None
                }
            },
            None => ConstantExpr::None
        }
    }
}