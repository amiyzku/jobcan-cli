use std::fmt::{Display, Formatter};

use serde::Deserialize;

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

impl StampType {
    pub fn expected_response(&self) -> Response {
        // Note: Ignore `Response.result` and `Response.state`
        match self {
            StampType::WorkStart => Response {
                current_status: "working".to_string(),
                ..Default::default()
            },
            StampType::WorkEnd => Response {
                current_status: "returned_home".to_string(),
                ..Default::default()
            },
            StampType::RestStart => Response {
                current_status: "resting".to_string(),
                ..Default::default()
            },
            StampType::RestEnd => Response {
                current_status: "working".to_string(),
                ..Default::default()
            },
        }
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Response {
    result: i32,
    state: i32,
    current_status: String,
}

impl Default for Response {
    fn default() -> Self {
        Response {
            result: 0,
            state: 0,
            current_status: "".to_string(),
        }
    }
}
