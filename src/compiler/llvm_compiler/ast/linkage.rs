

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

pub enum ExternLinkage { 
    ExternWeak,
	External
}