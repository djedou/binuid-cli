use super::Metadata;


pub struct MDFields {
    pub list: Vec<MDField>
}

pub enum MDField {
    Null,
    Metadata {
        data: Metadata
    }
}