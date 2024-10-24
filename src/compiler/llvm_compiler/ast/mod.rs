pub mod item;
pub mod top_level_entity;
pub mod types;
pub mod indirect_symbol_def;

mod utils;

pub use utils::*;

pub struct LlvmAst {
    pub items: Vec<self::item::Item>
}

impl LlvmAst {
    pub fn new() -> Self {
        LlvmAst {
            items: vec![]
        }
    }
}