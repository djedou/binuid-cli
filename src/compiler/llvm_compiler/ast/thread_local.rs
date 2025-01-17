use crate::compiler::{llvm_compiler::Rule, BuildFrom};
use binuid_shared_wasm::ast_bits::{
    simples::TLSModel,
    composes::ThreadLocal
};


impl BuildFrom for ThreadLocal {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> ThreadLocal {
        let mut thread_local = ThreadLocal {
            model: TLSModel::None
        };

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::TLSModel => {
                    thread_local.model = TLSModel::build_from(&inner_pair);
                },
                _ => {}
            }
        }

        thread_local
    }
}