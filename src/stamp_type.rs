use serde::Deserialize;

#[derive(Debug, PartialEq, Eq)]
pub enum StampType {
    ClockIn,
    ClockOut,
    StartBreak,
    EndBreak,
}

impl StampType {
    pub fn to_request_params(&self) -> String {
        match self {
            StampType::ClockIn => "work_start".to_string(),
            StampType::ClockOut => "work_end".to_string(),
            StampType::StartBreak => "rest_start".to_string(),
            StampType::EndBreak => "rest_end".to_string(),
        }
    }
}

impl StampType {
    pub fn expected_response(&self) -> Response {
        // Note: Ignore `Response.result` and `Response.state`
        match self {
            StampType::ClockIn => Response {
                current_status: "working".to_string(),
                ..Default::default()
            },
            StampType::ClockOut => Response {
                current_status: "returned_home".to_string(),
                ..Default::default()
            },
            StampType::StartBreak => Response {
                current_status: "resting".to_string(),
                ..Default::default()
            },
            StampType::EndBreak => Response {
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
