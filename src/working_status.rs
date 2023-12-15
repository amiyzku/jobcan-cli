use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum WorkingStatus {
    Working,
    NotWorking,
    Resting,
}

impl Display for WorkingStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WorkingStatus::Working => write!(f, "Working"),
            WorkingStatus::NotWorking => write!(f, "Not working"),
            WorkingStatus::Resting => write!(f, "Resting"),
        }
    }
}
