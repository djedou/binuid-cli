use super::OverflowFlag;

pub enum OpFlag {
    Overflows {
        flags: Vec<OverflowFlag>
    },
    Exact
}