use std::fmt::{Display, Formatter};

pub enum StampType {
    WorkStart,
    WorkEnd,
    RestStart,
    RestEnd,
}

impl Display for StampType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            StampType::WorkStart => write!(f, "work_start"),
            StampType::WorkEnd => write!(f, "work_end"),
            StampType::RestStart => write!(f, "rest_start"),
            StampType::RestEnd => write!(f, "rest_end"),
        }
    }
}
