use super::{TopLevelEntity, Comment};
use crate::compiler::{llvm_compiler::Rule, BuildFrom};

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
        /*match pair.into_inner().next() {
            Some(p) => {
                let (name, vis, item) = build_item_ast(p);
                ast.extend_module_content((name, ModuleContent::Item(vis, item)));
            }
            None => {}
        }*/
        /*
            match pair.as_rule() {
                    Rule::Item => {
        */

        println!("item output: {item:#?}");
        item
    }
}