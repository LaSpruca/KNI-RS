use serde::{Serialize, Deserialize};
use std::error::Error;
use std::fmt;

/// If the API returned an error
#[serde(rename = "NoticesResults")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NoticesError {
    #[serde(rename = "apiversion")]
    pub api_version: Option<String>,
    /// The portal version
    #[serde(rename = "portalversion")]
    pub portal_version: Option<String>,
    /// The access level of the auth key
    #[serde(rename = "AccessLevel")]
    pub access_level: i32,
    /// The error code
    #[serde(rename = "ErrorCode")]
    pub error_code: i32,
    /// The error
    #[serde(rename = "Error")]
    pub error: String
}

impl fmt::Display for NoticesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error: {} (error code: {})", self.error, self.error_code)
    }
}

impl Error for NoticesError {}

/// The response from the Kamar API
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NoticesResults {
    /// The API version
    #[serde(rename = "apiversion")]
    pub api_version: Option<String>,
    /// The portal version
    #[serde(rename = "portalversion")]
    pub portal_version: Option<String>,
    /// The access level of the auth key
    #[serde(rename = "AccessLevel")]
    pub access_level: i32,
    /// Error code, 0 if no error
    #[serde(rename = "ErrorCode")]
    pub error_code: i32,
    /// The date of the notices retrieved
    #[serde(rename = "NoticeDate")]
    pub notice_date: String,
    /// The total number of notices
    #[serde(rename = "NumberRecords")]
    pub number_records: i32,
    /// All of the meeting notices
    #[serde(rename = "MeetingNotices")]
    pub meeting_notices: MeetingNotices,
    /// All of the
    #[serde(rename = "GeneralNotices")]
    pub general_notices: GeneralNotices,
}

/// The struct for the meeting notices
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MeetingNotices {
    /// Number of meeting notices
    #[serde(rename = "NumberMeetingRecords")]
    pub number_meeting_records: String,

    /// The actual meeting notices
    #[serde(rename = "Meeting")]
    pub meeting: Option<Vec<Meeting>>,
}

/// One meeting notice
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Meeting {
    /// The index of the notice
    pub index: i32,
    /// Year levels/groups affected by notice
    #[serde(rename = "Level")]
    pub level: String,
    /// Subject/Title of the notice
    #[serde(rename = "Subject")]
    pub subject: String,
    /// Main content of the notice
    #[serde(rename = "Body")]
    pub body: String,
    /// The teacher that made the notice
    #[serde(rename = "Teacher")]
    pub teacher: String,
    /// The location affected students should meet
    #[serde(rename = "PlaceMeet")]
    pub place_meet: String,
    /// The date affected student should meet
    #[serde(rename = "DateMeet")]
    pub date_meet: String,
    #[serde(rename = "TimeMeet")]
    /// The time affected student should meet (can be values like lunch time etc.)
    pub time_meet: String,
}

/// The struct for the meeting notices
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GeneralNotices {
    /// The number of general notices
    #[serde(rename = "NumberGeneralRecords")]
    pub number_meeting_records: String,

    /// The actual meeting notices
    #[serde(rename = "General")]
    pub meeting: Option<Vec<General>>,
}

/// One general notice
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct General {
    /// The index of the notice
    pub index: i32,
    /// Year levels/groups affected by notice
    #[serde(rename = "Level")]
    pub level: String,
    /// Subject/Title of the notice
    #[serde(rename = "Subject")]
    pub subject: String,
    /// Main content of the notice
    #[serde(rename = "Body")]
    pub body: String,
    /// The teacher that made the notice
    #[serde(rename = "Teacher")]
    pub teacher: String,
}
