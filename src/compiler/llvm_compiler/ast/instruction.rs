use super::{StoreInst, FenceInst, CmpXchgInst, AtomicRMWInst, Ident, ValueInst};


#[derive(Debug)]
pub struct InstructionItem {
    start_comment: Option<String>,
    instruction: Box<Instruction>,
    end_comment: Option<String>
}


#[derive(Debug)]
pub enum Instruction {
    Store {
        inst: StoreInst
    },
	Fence {
        inst: FenceInst
    },
	CmpXchg {
        inst: CmpXchgInst
    },
	AtomicRMW {
        inst: AtomicRMWInst
    },
	LocalIdent {
        local_ident: Ident,
        value_inst: ValueInst
    },
	Value {
        value_inst: ValueInst
    }
}