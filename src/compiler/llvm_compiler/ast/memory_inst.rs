use crate::compiler::{llvm_compiler::Rule, BuildFrom};
use binuid_shared_wasm::ast_bits::instructions::{AllocaInst, MemoryInst, LoadInst, GetElementPtrInst};


impl BuildFrom for MemoryInst {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> MemoryInst {
        match pair.clone().into_inner().next() {
            Some(inner_pair) => {
                match inner_pair.as_rule() {
                    Rule::AllocaInst => MemoryInst::Alloca {
                        inst: AllocaInst::build_from(&inner_pair)
                    },
                    Rule::LoadInst => MemoryInst::Load {
                        inst: LoadInst::build_from(&inner_pair)
                    },
                    Rule::GetElementPtrInst => MemoryInst::GetElementPtr {
                        inst: GetElementPtrInst::build_from(&inner_pair)
                    },
                    _ => MemoryInst::None
                }
            },
            None => MemoryInst::None
        }
    }
}