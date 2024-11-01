use super::{MetadataAttachment, Comdat};

pub enum GlobalAttr {
    Section {
        section: String
    },
	Comdat {
        comdat: Comdat
    },
	Align {
        align: u32
    },
	MetadataAttachment {
        attachment: MetadataAttachment
    }
}