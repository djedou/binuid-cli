use super::{
    BinaryInst, BitwiseInst, VectorInst, AggregateInst, MemoryInst,
    ConversionInst, ICmpInst, FCmpInst, PhiInst, SelectInst, CallInst, 
    VAArgInst, LandingPadInst, CatchPadInst, CleanupPadInst, Terminator
};
use crate::compiler::{llvm_compiler::Rule, BuildFrom};



#[derive(Debug)]
pub enum ValueInst {
    None,
    Binary {
        inst: BinaryInst
    },
    Bitwise {
        inst: BitwiseInst
    },
    Vector {
        inst: VectorInst
    },
    Aggregate {
        inst: AggregateInst
    },
    Memory {
        inst: MemoryInst
    },

    Conversion {
        inst: ConversionInst
    },
    ICmp {
        inst: ICmpInst
    },
    FCmp {
        inst: FCmpInst
    },
    Phi {
        inst: PhiInst
    },
    Select {
        inst: SelectInst
    },
    Call {
        inst: CallInst
    },
    VAArg {
        inst: VAArgInst
    },
    LandingPad {
        inst: LandingPadInst
    },
    CatchPad {
        inst: CatchPadInst
    },
    CleanupPad {
        inst: CleanupPadInst
    },
    Terminator {
        term: Terminator
    }
}


impl BuildFrom for ValueInst {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> ValueInst {
        match pair.clone().into_inner().next() {
            Some(inner_pair) => {
                match inner_pair.as_rule() {
                    Rule::BinaryInst => ValueInst::Binary {
                        inst: BinaryInst::build_from(&inner_pair)
                    },
                    Rule::BitwiseInst => ValueInst::Bitwise {
                        inst: BitwiseInst::build_from(&inner_pair)
                    },
                    Rule::VectorInst => ValueInst::Vector {
                        inst: VectorInst::build_from(&inner_pair)
                    },
                    Rule::AggregateInst => ValueInst::Aggregate {
                        inst: AggregateInst::build_from(&inner_pair)
                    },
                    Rule::MemoryInst => ValueInst::Memory {
                        inst: MemoryInst::build_from(&inner_pair)
                    },
                    Rule::ConversionInst => ValueInst::Conversion {
                        inst: ConversionInst::build_from(&inner_pair)
                    },
                    Rule::ICmpInst => ValueInst::ICmp {
                        inst: ICmpInst::build_from(&inner_pair)
                    },
                    Rule::FCmpInst => ValueInst::FCmp {
                        inst: FCmpInst::build_from(&inner_pair)
                    },
                    Rule::PhiInst => ValueInst::Phi {
                        inst: PhiInst::build_from(&inner_pair)
                    },
                    Rule::SelectInst => ValueInst::Select {
                        inst: SelectInst::build_from(&inner_pair)
                    },
                    Rule::CallInst => ValueInst::Call {
                        inst: CallInst::build_from(&inner_pair)
                    },
                    Rule::VAArgInst => ValueInst::VAArg {
                        inst: VAArgInst::build_from(&inner_pair)
                    },
                    Rule::LandingPadInst => ValueInst::LandingPad {
                        inst: LandingPadInst::build_from(&inner_pair)
                    },
                    Rule::CatchPadInst => ValueInst::CatchPad {
                        inst: CatchPadInst::build_from(&inner_pair)
                    },
                    Rule::CleanupPadInst => ValueInst::CleanupPad {
                        inst: CleanupPadInst::build_from(&inner_pair)
                    },
                    Rule::Terminator => ValueInst::Terminator {
                        term: Terminator::build_from(&inner_pair)
                    },
                    _ => ValueInst::None
                }
            },
            None => ValueInst::None
        }
    }
}