use crate::compiler::{llvm_compiler::Rule, BuildFrom};
use binuid_shared_wasm::ast_bits::instructions::{Instruction, InstructionItem, StoreInst,
    FenceInst, CmpXchgInst, AtomicRMWInst, ValueInst
};
use binuid_shared_wasm::ast_bits::composes::Ident;


impl BuildFrom for InstructionItem {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> InstructionItem {
        let mut item = InstructionItem::new();

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::StartComment => {
                    item.start_comment = String::build_from(&inner_pair);
                },
                Rule::EndComment => {
                    item.end_comment = String::build_from(&inner_pair);
                },
                Rule::Instruction => {
                    item.instruction = Box::new(Instruction::build_from(&inner_pair));
                },
                _ => {}
            }
        }

        item
    }
}



impl BuildFrom for Instruction {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> Instruction {
        match pair.clone().into_inner().next() {
            Some(inner_pair) => {
                match inner_pair.as_rule() {
                    Rule::StoreInst => Instruction::Store {
                        inst: StoreInst::build_from(&inner_pair)
                    },
                    Rule::FenceInst => Instruction::Fence {
                        inst: FenceInst::build_from(&inner_pair)
                    },
                    Rule::CmpXchgInst => Instruction::CmpXchg {
                        inst: CmpXchgInst::build_from(&inner_pair)
                    },
                    Rule::AtomicRMWInst => Instruction::AtomicRMW {
                        inst: AtomicRMWInst::build_from(&inner_pair)
                    },
                    Rule::LocalInst => {
                        let mut ident = Ident::None;
                        let mut value = ValueInst::None;

                        for p in inner_pair.clone().into_inner() {
                            match p.as_rule() {
                                Rule::LocalIdent => {
                                    ident = Ident::build_from(&p)
                                },
                                Rule::ValueInstruction => {
                                    value = ValueInst::build_from(&p)
                                },
                                _ => {}
                            }
                        }

                        Instruction::LocalInst {
                            local_ident: ident,
                            value_inst: value
                        }
                    },
                    Rule::ValueInstruction => Instruction::Value {
                        value_inst: ValueInst::build_from(&inner_pair)
                    },
                    _ => Instruction::None
                }
            },
            None => Instruction::None
        }
    }
}