use super::{AllocaInst, LoadInst, GetElementPtrInst};

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