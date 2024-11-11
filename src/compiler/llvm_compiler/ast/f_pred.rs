use crate::compiler::{llvm_compiler::Rule, BuildFrom};


#[derive(Debug)]
pub enum FPred {
    None,
    False,
	True,
    Oeq,
    Oge,
    Ogt,
    Ole,
    Olt,
    One,
    Ord,
    Ueq,
    Uge,
    Ugt,
    Ule,
    Ult,
    Une,
    Uno
}


impl BuildFrom for FPred {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> FPred {
        match pair.clone().into_inner().next() {
            Some(inner_pair) => {
                match inner_pair.as_rule() {
                    Rule::False => FPred::False,
                    Rule::True => FPred::True,
                    Rule::Oeq => FPred::Oeq,
                    Rule::Oge => FPred::Oge,
                    Rule::Ogt => FPred::Ogt,
                    Rule::Ole => FPred::Ole,
                    Rule::Olt => FPred::Olt,
                    Rule::One => FPred::One,
                    Rule::Ord => FPred::Ord,
                    Rule::Ueq => FPred::Ueq,
                    Rule::Uge => FPred::Uge,
                    Rule::Ugt => FPred::Ugt,
                    Rule::Ule => FPred::Ule,
                    Rule::Ult => FPred::Ult,
                    Rule::Une => FPred::Une,
                    Rule::Uno => FPred::Uno,
                    _ => FPred::None
                }
            },
            None => FPred::None
        }
    }
}