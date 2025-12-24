pub mod client;
pub mod models;
pub mod parser;

pub use client::fetch_calendar;
pub use models::{CalendarEvent, EventTime};
pub use parser::parse_calendar_xml;

use anyhow::Context;
use anyhow::Result;
use chrono::NaiveDate;

// need to make a recursive function that takes in a start and end date. and fetches all events between those dates
// it has to be recursive because the API ends due to size limits
pub async fn fetch_events_recursive(
    base_url: &str,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<Vec<CalendarEvent>> {
    let mut all_events = Vec::new();
    let mut current_start = start_date;

    loop {
        // fetch events
        let events = fetch_calendar(base_url, current_start, end_date).await?;
        let events = parse_calendar_xml(events)?;

        if events.is_empty() {
            break;
        }

        let last_event_date = events
            .last()
            .map(|e| e.start.date())
            .context("Failed to get last date")?;

        all_events.extend(events);

        // Stop if we've reached the end date
        if last_event_date >= end_date {
            break;
        }

        // Continue from the same day as the last event to avoid missing events
        // The deduplication below will handle any duplicates
        current_start = last_event_date;
    }

    // deduplicate events by id
    all_events.sort_by(|a, b| a.event_id.cmp(&b.event_id));
    all_events.dedup_by(|a, b| a.event_id == b.event_id);

    // sort events by start date
    all_events.sort_by(|a, b| a.start.cmp(&b.start));

    Ok(all_events)
}
