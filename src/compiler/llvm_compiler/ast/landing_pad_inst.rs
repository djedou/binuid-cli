use super::{Type, Clause, MetadataAttachment};


#[derive(Debug)]
pub struct LandingPadInst { 
    pub type_: Type,
    pub opt_cleanup: bool,
    pub clauses: Vec<Clause>, 
    pub metadata_attachments: Vec<MetadataAttachment>
}