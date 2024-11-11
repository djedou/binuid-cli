use crate::compiler::{llvm_compiler::Rule, BuildFrom};


#[derive(Debug)]
pub enum Linkage {
    None,
    Appending,
	AvailableExternally,
	Common,
	Internal,
	Linkonce,
	linkonceOdr,
	Private,
	Weak,
	WeakOdr,
}

#[derive(Debug)]
pub enum ExternLinkage {
    None, 
    ExternWeak,
	External
}


impl BuildFrom for Linkage {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> Linkage {
        match pair.clone().into_inner().next() {
            Some(inner_pair) => {
                match inner_pair.as_rule() {
                    Rule::Appending => Linkage::Appending,
                    Rule::AvailableExternally => Linkage::AvailableExternally,
                    Rule::Common => Linkage::Common,
                    Rule::Internal => Linkage::Internal,
                    Rule::Linkonce => Linkage::Linkonce,
                    Rule::linkonceOdr => Linkage::linkonceOdr,
                    Rule::Private => Linkage::Private,
                    Rule::Weak => Linkage::Weak,
                    Rule::WeakOdr => Linkage::WeakOdr,
                    _ => Linkage::None
                }
            },
            None => Linkage::None
        }
    }
}

impl BuildFrom for ExternLinkage {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> ExternLinkage {
        match pair.clone().into_inner().next() {
            Some(inner_pair) => {
                match inner_pair.as_rule() {
                    Rule::ExternWeak => ExternLinkage::ExternWeak,
                    Rule::External => ExternLinkage::External,
                    _ => ExternLinkage::None
                }
            },
            None => ExternLinkage::None
        }
    }
}