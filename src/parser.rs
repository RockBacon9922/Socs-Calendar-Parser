use crate::models::{CalendarEvent, CalendarEventXml, EventTime, SOCSCalendar};
use anyhow::{Context, Result};
use chrono::{NaiveDate, NaiveTime};

/// Parse XML calendar data into structured events
pub fn parse_calendar_xml(xml_data: String) -> Result<Vec<CalendarEvent>> {
    let calendar: SOCSCalendar = serde_xml_rs::from_str(&xml_data.to_string())
        .context("Failed to parse XML calendar data")?;

    calendar.events.into_iter().map(parse_event).collect()
}

fn parse_event(event: CalendarEventXml) -> Result<CalendarEvent> {
    let start_date = parse_date(&event.start_date)
        .context(format!("Failed to parse start date: {}", event.start_date))?;

    let end_date = parse_date(&event.end_date)
        .context(format!("Failed to parse end date: {}", event.end_date))?;

    let start = parse_event_time(start_date, &event.start_time)
        .context(format!("Failed to parse start time: {}", event.start_time))?;

    let end = if let Some(end_time_str) = &event.end_time {
        if !end_time_str.trim().is_empty() {
            parse_event_time(end_date, end_time_str)
                .context(format!("Failed to parse end time: {}", end_time_str))?
        } else {
            // If end time is empty, use end of day or match start
            if start.is_all_day() {
                EventTime::AllDay(end_date)
            } else {
                // Default to 1 hour after start if no end time provided
                if let EventTime::Specific { date: _, time } = &start {
                    let end_time = time.overflowing_add_signed(chrono::Duration::hours(1)).0;
                    EventTime::Specific {
                        date: end_date,
                        time: end_time,
                    }
                } else {
                    EventTime::AllDay(end_date)
                }
            }
        }
    } else {
        // No end time at all, assume same as start
        if start.is_all_day() {
            EventTime::AllDay(end_date)
        } else {
            start.clone()
        }
    };

    // Parse categories - comma-separated
    let categories: Vec<String> = event
        .category
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    Ok(CalendarEvent {
        event_id: event.event_id,
        title: event.title,
        description: event.description,
        location: event.location,
        categories,
        start,
        end,
    })
}

/// Parse date in format "10/12/2025" (DD/MM/YYYY)
fn parse_date(date_str: &str) -> Result<NaiveDate> {
    let parts: Vec<&str> = date_str.split('/').collect();

    if parts.len() != 3 {
        anyhow::bail!("Invalid date format: {}", date_str);
    }

    let day: u32 = parts[0]
        .parse()
        .context(format!("Invalid day: {}", parts[0]))?;
    let month: u32 = parts[1]
        .parse()
        .context(format!("Invalid month: {}", parts[1]))?;
    let year: i32 = parts[2]
        .parse()
        .context(format!("Invalid year: {}", parts[2]))?;

    NaiveDate::from_ymd_opt(year, month, day)
        .context(format!("Invalid date: {}/{}/{}", day, month, year))
}

/// Parse event time - can be "All Day" or "HH:MM" format
fn parse_event_time(date: NaiveDate, time_str: &str) -> Result<EventTime> {
    let time_str = time_str.trim();

    if time_str.eq_ignore_ascii_case("all day") || time_str.is_empty() {
        return Ok(EventTime::AllDay(date));
    }

    let time = NaiveTime::parse_from_str(time_str, "%H:%M")
        .context(format!("Failed to parse time: {}", time_str))?;

    Ok(EventTime::Specific { date, time })
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Datelike, Timelike};

    #[test]
    fn test_parse_date() {
        let date = parse_date("10/12/2025").unwrap();
        assert_eq!(date.day(), 10);
        assert_eq!(date.month(), 12);
        assert_eq!(date.year(), 2025);
    }

    #[test]
    fn test_parse_all_day_time() {
        let date = NaiveDate::from_ymd_opt(2025, 12, 10).unwrap();
        let event_time = parse_event_time(date, "All Day").unwrap();
        assert!(event_time.is_all_day());
    }

    #[test]
    fn test_parse_specific_time() {
        let date = NaiveDate::from_ymd_opt(2025, 12, 10).unwrap();
        let event_time = parse_event_time(date, "08:30").unwrap();
        assert!(!event_time.is_all_day());

        if let EventTime::Specific { time, .. } = event_time {
            assert_eq!(time.hour(), 8);
            assert_eq!(time.minute(), 30);
        } else {
            panic!("Expected specific time");
        }
    }
}
