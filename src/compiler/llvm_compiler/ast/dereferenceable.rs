
#[derive(Debug)]
pub struct Dereferenceable {
    pub item: DereferenceableItem,
    pub value: u32
}


#[derive(Debug)]
pub enum DereferenceableItem { 
    Nullable,
    NonNullable
}