
#[derive(Debug)]
pub struct TargetDefinition {
    pub kink: TargetKind,
    pub data: String
}

#[derive(Debug)]
pub enum TargetKind {
    Datalayout,
    Triple
}