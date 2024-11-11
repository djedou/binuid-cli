use crate::compiler::{llvm_compiler::Rule, BuildFrom};


#[derive(Debug)]
pub struct InlineAsm { 
    pub side_effect: bool,
    pub align_stack: bool,
    pub intel_dialect: bool, 
    pub lhs_string: String,
    pub rhs_string: String
}


impl BuildFrom for InlineAsm {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> InlineAsm {
        let mut inline_asm = InlineAsm { 
            side_effect: false,
            align_stack: false,
            intel_dialect: false, 
            lhs_string: String::with_capacity(0),
            rhs_string: String::with_capacity(0)
        };
        
        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::SideEffect => {
                    inline_asm.side_effect = true;
                },
                Rule::AlignStack => {
                    inline_asm.align_stack = true;
                },
                Rule::IntelDialect => {
                    inline_asm.intel_dialect = true;
                },
                Rule::LhsStringLit => {
                    inline_asm.lhs_string = String::build_from(&inner_pair);
                },
                Rule::RhsStringLit => {
                    inline_asm.rhs_string = String::build_from(&inner_pair);
                },
                _ => {}
            }
        }

        inline_asm
    }
}