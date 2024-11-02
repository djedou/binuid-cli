use pest::{self, Parser, error::LineColLocation};
use super::{LlvmAst, Item};
use crate::compiler::BuildFrom;


#[derive(pest_derive::Parser)]
#[grammar = "./src/compiler/llvm_compiler/grammar/keywords.pest"]
#[grammar = "./src/compiler/llvm_compiler/grammar/llvm.pest"]
struct LLParser;

pub fn parse_ll(source: &str) -> LlvmAst {
    
    let mut ast = LlvmAst::new();
    match LLParser::parse(Rule::Module, source) {
        Ok(pairs) => {
            for pair in pairs {
                match pair.as_rule() {
                    Rule::Item => {
                        ast.items.push(Item::build_from(&pair));
                        /*match pair.clone().into_inner().next() {
                            Some(p) => {
                                let (name, vis, item) = build_item_ast(p);
                                ast.extend_module_content((name, ModuleContent::Item(vis, item)));
                            }
                            None => {}
                        }*/
                    },
                    _ => {}
                }
            }
        },
        Err(e) => {
            println!("Err: {:#?}", e);
            let LineColLocation::Pos((line, col)) = e.line_col else {
                return LlvmAst::new();;
            };

            println!("Parsing Error: line: {:?} and column: {:?}", line, col);
        }
    }
    
    ast
}