use super::{AllocaInst, LoadInst, GetElementPtrInst};


#[derive(Debug)]
pub enum MemoryInst {
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