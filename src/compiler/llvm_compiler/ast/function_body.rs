use super::{UseListOrder, InstructionItem, Terminator};


pub struct FunctionBody { 
    basic_block_list: Vec<BasicBlock>,
    use_list_orders: Vec<UseListOrder>
}

pub struct BasicBlock { 
    label_ident: String,
    comments: Vec<String>,
    instructions: Vec<InstructionItem>,
    terminator: Option<Terminator>
}