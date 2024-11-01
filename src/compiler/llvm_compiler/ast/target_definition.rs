pub struct TargetDefinition {
    pub kink: TargetKind,
    pub data: String
}


pub enum TargetKind {
    Datalayout,
    Triple
}