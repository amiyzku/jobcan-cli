use std::fmt::Display;

use serde::Deserialize;

#[derive(Debug, PartialEq, Eq)]
pub enum Stamp {
    ClockIn,
    ClockOut,
    StartBreak,
    EndBreak,
}

impl Display for Stamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Stamp::ClockIn => "ClockIn",
            Stamp::ClockOut => "ClockOut",
            Stamp::StartBreak => "StartBreak",
            Stamp::EndBreak => "EndBreak",
        };
        write!(f, "{}", s)
    }
}

impl Stamp {
    pub fn to_request_params(&self) -> String {
        match self {
            Stamp::ClockIn => "work_start".to_string(),
            Stamp::ClockOut => "work_end".to_string(),
            Stamp::StartBreak => "rest_start".to_string(),
            Stamp::EndBreak => "rest_end".to_string(),
        }
    }
    pub fn expected_response(&self) -> Response {
        // Note: Ignore `Response.result` and `Response.state`
        match self {
            Stamp::ClockIn => Response {
                current_status: "working".to_string(),
                ..Default::default()
            },
            Stamp::ClockOut => Response {
                current_status: "returned_home".to_string(),
                ..Default::default()
            },
            Stamp::StartBreak => Response {
                current_status: "resting".to_string(),
                ..Default::default()
            },
            Stamp::EndBreak => Response {
                current_status: "working".to_string(),
                ..Default::default()
            },
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Response {
    #[allow(dead_code)] // Note: Use json deserialization
    result: i32,
    #[allow(dead_code)] // Note: Use json deserialization
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

impl PartialEq for Response {
    fn eq(&self, other: &Self) -> bool {
        // Note: Ignore `self.result` and `self.state`
        self.current_status == other.current_status
    }
}
