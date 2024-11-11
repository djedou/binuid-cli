use super::{AllocaInst, LoadInst, GetElementPtrInst};
use crate::compiler::{llvm_compiler::Rule, BuildFrom};

#[derive(Debug)]
pub enum MemoryInst {
    None,
    Alloca {
        inst: AllocaInst
    },
    Load {
        inst: LoadInst
    },
    GetElementPtr {
        inst: GetElementPtrInst
    }
}


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