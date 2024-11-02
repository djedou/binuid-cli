use super::{MetadataId, MDTuple};


#[derive(Debug)]
pub struct MetadataDef {
    pub id: MetadataId,
    pub distinct: bool,
    pub md_tuple: Option<MDTuple>
}