use super::{
    BinaryInst, BitwiseInst, VectorInst, AggregateInst, MemoryInst,
    ConversionInst, ICmpInst, FCmpInst, PhiInst, SelectInst, CallInst, 
    VAArgInst, LandingPadInst, CatchPadInst, CleanupPadInst, Terminator
};

pub enum ValueInst {
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