use super::{Type, Clause, MetadataAttachment};
use crate::compiler::{llvm_compiler::Rule, BuildFrom};


#[derive(Debug)]
pub struct LandingPadInst { 
    pub type_: Type,
    pub opt_cleanup: bool,
    pub clauses: Vec<Clause>, 
    pub metadata_attachments: Vec<MetadataAttachment>
}



impl LandingPadInst {
    pub fn new() -> LandingPadInst {
        LandingPadInst {
            type_: Type::None,
            opt_cleanup: false,
            clauses: vec![],
            metadata_attachments: vec![]
        }
    }
}


impl BuildFrom for LandingPadInst {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> LandingPadInst {
        let mut item = LandingPadInst::new();

        for inner_pair in pair.clone().into_inner() {
            match inner_pair.as_rule() {
                Rule::Type => {
                    item.type_ = Type::build_from(&inner_pair);
                },
                Rule::OptCleanup => {
                    item.opt_cleanup = true;
                },
                Rule::Clauses => {
                    for p in inner_pair.clone().into_inner() {
                        match p.as_rule() {
                            Rule::Clause => {
                                item.clauses.push(Clause::build_from(&p));
                            },
                            _ => {}
                        }
                    }
                },
                Rule::MetadataAttachment => {
                    item.metadata_attachments.push(MetadataAttachment::build_from(&inner_pair));
                },
                _ => {}
            }
        }

        item
    }
}