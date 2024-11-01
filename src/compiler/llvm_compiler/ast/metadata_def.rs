use super::{MetadataId, MDTuple};

pub struct MetadataDef {
    pub id: MetadataId,
    pub distinct: bool,
    pub md_tuple: Option<MDTuple>
}