use super::{FloatLit, StructConst, ArrayConst, VectorConst, Ident, BlockAddressConst, ConstantExpr};
use crate::compiler::{llvm_compiler::Rule, BuildFrom};

#[derive(Debug)]
pub enum Constant {
    Bool {
        value: bool
    },
	Int {
        value: i64
    },
	Float {
        value: FloatLit
    },
	Null,
	None,
	Struct {
        value: StructConst
    },
	Array {
        value: ArrayConst
    },
	CharArray {
        value: String
    },
	Vector {
        value: VectorConst
    },
	ZeroInitializer,
	GlobalIdent {
        ident: Ident
    },
	Undef,
	BlockAddress {
        address: BlockAddressConst
    },
	Expr {
        expr: Box<ConstantExpr>
    }
}


impl BuildFrom for Constant {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> Constant {
        match pair.clone().into_inner().next() {
            Some(inner_pair) => {
                match inner_pair.as_rule() {
                    Rule::BoolConst => Constant::Bool {
                        value: inner_pair.as_str().parse::<bool>().map_or(false, |d| d.clone())
                    },
                    Rule::IntConst => Constant::Int {
                        value: inner_pair.as_str().parse::<i64>().map_or(0, |d| d.clone())
                    },
                    Rule::FloatConst => Constant::Float {
                        value: FloatLit::build_from(&inner_pair)
                    },
                    Rule::NullConst => Constant::Null,
                    Rule::NoneConst => Constant::None,
                    Rule::StructConst => Constant::Struct {
                        value: StructConst::build_from(&inner_pair)
                    },
                    Rule::ArrayConst => Constant::Array {
                        value: ArrayConst::build_from(&inner_pair)
                    },
                    Rule::CharArrayConst => {
                        match inner_pair.into_inner().next() {
                            Some(p) => {
                                match p.as_rule() {
                                    Rule::StringLit => {
                                        Constant::CharArray {
                                            value: String::build_from(&p)
                                        }
                                    },
                                    _ => Constant::None
                                }
                            },
                            None => Constant::None
                        }
                    },
                    Rule::VectorConst => Constant::Vector {
                        value: VectorConst::build_from(&inner_pair)
                    },
                    Rule::ZeroInitializerConst => Constant::ZeroInitializer,
                    Rule::GlobalIdent => Constant::GlobalIdent {
                        ident: Ident::build_from(&inner_pair)
                    },
                    Rule::UndefConst => Constant::Undef,
                    Rule::BlockAddressConst => Constant::BlockAddress {
                        address: BlockAddressConst::build_from(&inner_pair)
                    },
                    Rule::ConstantExpr => Constant::Expr {
                        expr: Box::new(ConstantExpr::build_from(&inner_pair))
                    },
                    _ => Constant::None
                }
            },
            None => Constant::None
        }
    }
}