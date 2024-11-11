use super::{Alignment, Dereferenceable, Sret, Range};
use crate::compiler::{llvm_compiler::Rule, BuildFrom};

#[derive(Debug)]
pub enum ParamAttr { 
    None,
    Align {
        align: Alignment
    },
	Dereferenceable {
        der: Dereferenceable
    },
	String {
        value: String
    },
	Byval,
	Inalloca,
	Inreg,
	Nest,
	Noalias,
	Nocapture,
	Nonnull,
	Readnone,
	Readonly,
	Returned,
	Signext,
	Sret {
        sret: Sret
    },
	Swifterror,
	Swiftself,
	Writeonly,
	Zeroext,
    Noundef,
    DeadOnUnwind,
    Writable,
    Immarg,
    Allocalign,
    Allocptr,
    Range {
        range: Range
    }
}


impl BuildFrom for ParamAttr {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> ParamAttr {
        match pair.clone().into_inner().next() {
            Some(inner_pair) => {
                match inner_pair.as_rule() {
                    Rule::Alignment => ParamAttr::Align {
                        align: Alignment::build_from(&inner_pair)
                    },
                    Rule::Dereferenceable => ParamAttr::Dereferenceable {
                        der: Dereferenceable::build_from(&inner_pair)
                    },
                    Rule::StringLit => ParamAttr::String {
                        value: inner_pair.as_str().trim_matches('\"').to_string()
                    },
                    Rule::Byval => ParamAttr::Byval,
                    Rule::Inalloca => ParamAttr::Inalloca,
                    Rule::Inreg => ParamAttr::Inreg,
                    Rule::Nest => ParamAttr::Nest,
                    Rule::Noalias => ParamAttr::Noalias,
                    Rule::Nocapture => ParamAttr::Nocapture,
                    Rule::Nonnull => ParamAttr::Nonnull,
                    Rule::Readnone => ParamAttr::Readnone,
                    Rule::Readonly => ParamAttr::Readonly,
                    Rule::Returned => ParamAttr::Returned,
                    Rule::Signext => ParamAttr::Signext,
                    Rule::Sret => ParamAttr::Sret {
                        sret: Sret::build_from(&inner_pair)
                    },
                    Rule::Swifterror => ParamAttr::Swifterror,
                    Rule::Swiftself => ParamAttr::Swiftself,
                    Rule::Writeonly => ParamAttr::Writeonly,
                    Rule::Zeroext => ParamAttr::Zeroext,
                    Rule::Noundef => ParamAttr::Noundef,
                    Rule::DeadOnUnwind => ParamAttr::DeadOnUnwind,
                    Rule::Writable => ParamAttr::Writable,
                    Rule::Immarg => ParamAttr::Immarg,
                    Rule::Allocalign => ParamAttr::Allocalign,
                    Rule::Allocptr => ParamAttr::Allocptr,
                    Rule::Range => ParamAttr::Range {
                        range: Range::build_from(&inner_pair)
                    },
                    _ => ParamAttr::None
                }
            },
            None => ParamAttr::None
        }
    }
}
