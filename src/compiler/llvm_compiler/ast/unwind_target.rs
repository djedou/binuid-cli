use super::Ident;

#[derive(Debug)]
pub enum UnwindTarget { 
    ToCaller,
	LocalLabel {
        label: Ident
    }
}