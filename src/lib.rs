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
/// Recursively fetches all calendar events between the given start and end dates.
///
/// This function handles the limitation of the SOCS API which may truncate results due to size limits.
/// It fetches events in chunks, starting from the given start date and continuing until all events
/// within the date range are retrieved. The function automatically handles pagination by using
/// the date of the last retrieved event as the starting point for the next request.
///
/// Events are deduplicated by ID and sorted by start time before being returned.
///
/// # Arguments
///
/// * `base_url` - The base URL for the SOCS calendar API which you are given when you create a key
/// * `start_date` - The start date for the event range (inclusive)
/// * `end_date` - The end date for the event range (inclusive)
///
/// # Returns
///
/// Returns a `Result` containing a vector of `CalendarEvent`s if successful, or an error if the
/// fetching or parsing fails.
///
/// # Examples
///
/// ```rust,no_run
/// use chrono::NaiveDate;
/// use socs_calendar_parser::fetch_events_recursive;
///
/// # async fn example() -> anyhow::Result<()> {
/// let start = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
/// let end = NaiveDate::from_ymd_opt(2025, 12, 31).unwrap();
/// let events = fetch_events_recursive("https://www.socscms.com/socs/xml/SOCScalendar.ashx?ID={}key={}", start, end).await?;
/// println!("Found {} events", events.len());
/// # Ok(())
/// # }
/// ```
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
