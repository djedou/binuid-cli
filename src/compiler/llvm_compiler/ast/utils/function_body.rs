
pub struct FunctionBody { 
    BasicBlockList: Vec<BasicBlock>,
    UseListOrders: Vec<UseListOrder>
}

pub struct BasicBlock { 
    LabelIdent: String,
    comments: Vec<String>,
    Instructions: Vec<InstructionItem>,
    Terminator: Option<Terminator>
}