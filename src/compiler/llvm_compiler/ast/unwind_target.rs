use super::Ident;

pub enum UnwindTarget { 
    ToCaller,
	LocalLabel {
        label: Ident
    }
}