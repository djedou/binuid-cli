use crate::compiler::{llvm_compiler::Rule, BuildFrom};

#[derive(Debug)]
pub enum BinOp {
    None,
    Add,
    And,
    Max,
    Min,
    Nand,
    Or,
    Sub,
    Umax,
    Umin,
    Xchg,
    Xor
}



impl BuildFrom for BinOp {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> BinOp {
        match pair.clone().into_inner().next() {
            Some(inner_pair) => {
                match inner_pair.as_rule() {
                    Rule::Add => BinOp::Add,
                    Rule::And => BinOp::And,
                    Rule::Max => BinOp::Max,
                    Rule::Min => BinOp::Min,
                    Rule::Nand => BinOp::Nand,
                    Rule::Or => BinOp::Or,
                    Rule::Sub => BinOp::Sub,
                    Rule::Umax => BinOp::Umax,
                    Rule::Umin => BinOp::Umin,
                    Rule::Xchg => BinOp::Xchg,
                    Rule::Xor => BinOp::Xor,
                    _ => BinOp::None
                }
            },
            None => BinOp::None
        }
    }
}