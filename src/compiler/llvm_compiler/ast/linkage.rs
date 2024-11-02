
#[derive(Debug)]
pub enum Linkage {
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
    ExternWeak,
	External
}