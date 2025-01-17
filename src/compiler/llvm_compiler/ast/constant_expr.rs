use binuid_shared_wasm::ast_bits::exprs::{ConstantExpr, BinaryExpr, BitwiseExpr, VectorExpr, AggregateExpr, MemoryExpr, ConversionExpr, OtherExp};
use crate::compiler::{llvm_compiler::Rule, BuildFrom};


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