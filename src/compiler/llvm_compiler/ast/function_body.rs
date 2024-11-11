use super::{UseListOrder, InstructionItem, Terminator, Comment};
use crate::compiler::{llvm_compiler::Rule, BuildFrom};

#[derive(Debug)]
pub struct FunctionBody { 
    basic_block_list: Vec<BasicBlock>,
    use_list_orders: Vec<UseListOrder>
}

#[derive(Debug)]
pub struct BasicBlock { 
    label_ident: String,
    comments: Vec<Comment>,
    instructions: Vec<InstructionItem>,
    terminator: Terminator
}

impl FunctionBody {
    pub fn new() -> FunctionBody {
        FunctionBody { 
            basic_block_list: vec![],
            use_list_orders: vec![]
        }
    }
}

impl BasicBlock {
    pub fn new() -> BasicBlock {
        BasicBlock { 
            label_ident: String::with_capacity(0),
            comments: vec![],
            instructions: vec![],
            terminator: Terminator::None
        }
    }
}


impl BuildFrom for FunctionBody {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> FunctionBody {
        let mut function_body = FunctionBody::new();

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::BasicBlock => {
                    function_body.basic_block_list.push(BasicBlock::build_from(&inner_pair));
                },
                Rule::UseListOrder => {
                    function_body.use_list_orders.push(UseListOrder::build_from(&inner_pair));
                },
                _ => {}
            }
        }
        
        function_body
    }
}


impl BuildFrom for BasicBlock {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> BasicBlock {
        let mut basick_block = BasicBlock::new();

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::LabelIdent => {
                    basick_block.label_ident = String::build_from(&inner_pair);
                },
                Rule::Comment => {
                    basick_block.comments.push(Comment::build_from(&inner_pair));
                },
                Rule::InstructionItem => {
                    basick_block.instructions.push(InstructionItem::build_from(&inner_pair));
                },
                Rule::Terminator => {
                    basick_block.terminator = Terminator::build_from(&inner_pair);
                },
                _ => {}
            }
        }
        
        basick_block
    }
}
