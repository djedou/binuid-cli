

#[derive(Debug)]
pub enum SelectionKind { 
    Any,
	ExactMatch,
	Largest,
	Noduplicates,
	Samesize,
}