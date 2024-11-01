use pest::{self, Parser, error::LineColLocation};
use super::LlvmAst;


#[derive(pest_derive::Parser)]
#[grammar = "./src/compiler/llvm_compiler/grammar/keywords.pest"]
#[grammar = "./src/compiler/llvm_compiler/grammar/llvm.pest"]
struct LLParser;

pub fn parse_ll(source: &str) -> LlvmAst {
    
    let mut ast = LlvmAst::new();
    match LLParser::parse(Rule::Module, source) {
        Ok(pairs) => {
            println!("pairs: {:#?}", pairs);
            /*for pair in pairs {
                match pair.as_rule() {
                    Rule::Item => {
                        match pair.clone().into_inner().next() {
                            Some(p) => {
                                let (name, vis, item) = build_item_ast(p);
                                ast.extend_module_content((name, ModuleContent::Item(vis, item)));
                            }
                            None => {}
                        }
                    },
                    Rule::BlockExpression => {
                        match build_block_expression_ast(pair.clone()) {
                            (_, ExprWithBlock::BlockExpr(expr)) => {
                                ast.extend_module_content(("anonymous".to_string(), ModuleContent::BlockExpr(expr)));
                            },
                            (_, _) => {}
                        }
                    },
                    Rule::LogStdStatement => {
                        //ast.extend_module_content(&build_ast_module_content_from_log_std(pair.clone()));
                    },
                    Rule::LogWebStatement => {
                        //ast.extend_module_content(&build_ast_module_content_from_log_web(pair.clone()));
                    },
                    _ => {}
                }
            }*/
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