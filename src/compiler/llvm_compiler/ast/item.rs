use super::{TopLevelEntity, Comment};
use crate::compiler::{llvm_compiler::{IntoFrame, Rule, LlvmAst}, BuildFrom};
use binuid_shared_wasm::vm::Frame;


#[derive(Debug)]
pub struct Item {
    pub comments: Vec<Comment>,
    pub top_level_entity: TopLevelEntity
}

impl BuildFrom for Item {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> Item {
        let mut item = Item {
            comments: vec![], 
            top_level_entity: TopLevelEntity::None
        };

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::Comment => {
                    item.comments.push(Comment::build_from(&inner_pair));
                },
                Rule::TopLevelEntity => {
                    item.top_level_entity = TopLevelEntity::build_from(&inner_pair);
                },
                _ => {}
            }
        }
        
        item
    }
}

impl IntoFrame for Item {
    fn into_frame(&self, ast: &LlvmAst, frame: &mut Frame) {
        self.top_level_entity.into_frame(&ast, frame);
    }
}