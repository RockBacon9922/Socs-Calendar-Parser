use chrono::{NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Deserialize)]
pub struct SOCSCalendar {
    #[serde(rename = "CalendarEvent", default)]
    pub events: Vec<CalendarEventXml>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CalendarEventXml {
    #[serde(rename = "EventID")]
    pub event_id: String,

    #[serde(rename = "StartDate")]
    pub start_date: String,

    #[serde(rename = "EndDate")]
    pub end_date: String,

    #[serde(rename = "StartTime")]
    pub start_time: String,

    #[serde(rename = "EndTime")]
    pub end_time: Option<String>,

    #[serde(rename = "Title")]
    pub title: String,

    #[serde(rename = "Description")]
    pub description: Option<String>,

    #[serde(rename = "Location")]
    pub location: String,

    #[serde(rename = "Category")]
    pub category: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarEvent {
    pub event_id: String,
    pub title: String,
    pub description: Option<String>,
    pub location: String,
    pub categories: Vec<String>,
    pub start: EventTime,
    pub end: EventTime,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum EventTime {
    AllDay(NaiveDate),
    Specific { date: NaiveDate, time: NaiveTime },
}

impl EventTime {
    pub fn date(&self) -> NaiveDate {
        match self {
            EventTime::AllDay(date) => *date,
            EventTime::Specific { date, .. } => *date,
        }
    }

    pub fn is_all_day(&self) -> bool {
        matches!(self, EventTime::AllDay(_))
    }
}

impl fmt::Display for EventTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EventTime::AllDay(date) => write!(f, "{} (All Day)", date.format("%d %b %Y")),
            EventTime::Specific { date, time } => {
                write!(f, "{} at {}", date.format("%d %b %Y"), time.format("%H:%M"))
            }
        }
    }
}
