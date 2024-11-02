use super::OverflowFlag;


#[derive(Debug)]
pub enum OpFlag {
    Overflows {
        flags: Vec<OverflowFlag>
    },
    Exact
}