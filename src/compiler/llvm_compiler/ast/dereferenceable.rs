
pub struct Dereferenceable {
    pub item: DereferenceableItem,
    pub value: u32
}

pub enum DereferenceableItem { 
    Nullable,
    NonNullable
}