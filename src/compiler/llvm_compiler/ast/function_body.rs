use crate::compiler::{llvm_compiler::Rule, BuildFrom};
use binuid_shared_wasm::ast_bits::composes::{BasicBlock, FunctionBody, UseListOrder, Comment,
    Terminator
};
use binuid_shared_wasm::ast_bits::instructions::InstructionItem;


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
