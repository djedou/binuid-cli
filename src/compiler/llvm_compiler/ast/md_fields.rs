use super::Metadata;

#[derive(Debug)]
pub struct MDFields {
    pub list: Vec<MDField>
}


#[derive(Debug)]
pub enum MDField {
    Null,
    Metadata {
        data: Metadata
    }
}