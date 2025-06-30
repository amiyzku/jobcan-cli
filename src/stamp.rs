use std::fmt::Display;

use serde::Deserialize;

#[derive(Debug, PartialEq, Eq)]
pub enum Stamp {
    Auto,
    ClockIn,
    ClockOut,
    StartBreak,
    EndBreak,
}

impl Display for Stamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:#?}")
    }
}

impl Stamp {
    pub fn to_request_params(&self) -> String {
        match self {
            Stamp::Auto => "DEF".to_string(),
            Stamp::ClockIn => "work_start".to_string(),
            Stamp::ClockOut => "work_end".to_string(),
            Stamp::StartBreak => "rest_start".to_string(),
            Stamp::EndBreak => "rest_end".to_string(),
        }
    }
    pub fn expected_response(&self) -> Option<Response> {
        // Note: Ignore `Response.result` and `Response.state`
        match self {
            Stamp::Auto => None,
            Stamp::ClockIn => Some(Response {
                current_status: CurrentStatus::Working,
                ..Default::default()
            }),
            Stamp::ClockOut => Some(Response {
                current_status: CurrentStatus::ReturnedHome,
                ..Default::default()
            }),
            Stamp::StartBreak => Some(Response {
                current_status: CurrentStatus::Resting,
                ..Default::default()
            }),
            Stamp::EndBreak => Some(Response {
                current_status: CurrentStatus::Working,
                ..Default::default()
            }),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum IntOrString {
    #[allow(dead_code)] // Note: Use json deserialization
    Int(i32),
    #[allow(dead_code)] // Note: Use json deserialization
    Str(String),
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum CurrentStatus {
    Working,
    ReturnedHome,
    Resting,
}

#[derive(Debug, Deserialize)]
pub struct Response {
    #[allow(dead_code)] // Note: Use json deserialization
    result: i32,
    #[allow(dead_code)] // Note: Use json deserialization
    state: IntOrString,
    current_status: CurrentStatus,
}

impl Default for Response {
    fn default() -> Self {
        Response {
            result: 0,
            state: IntOrString::Int(0),
            current_status: CurrentStatus::Working,
        }
    }
}

impl PartialEq for Response {
    fn eq(&self, other: &Self) -> bool {
        // Note: Ignore `self.result` and `self.state`
        self.current_status == other.current_status
    }
}
