
#[derive(Debug)]
pub enum AtomicOrdering { 
    AcqRel,
    Acquire,
    Monotonic,
    Release,
    SeqCst,
    Unordered
}