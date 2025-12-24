# SOCS Calendar Parser

A Rust library for fetching and parsing calendar events from SOCS (School Organisation and Curriculum System) APIs.

## Overview

This library provides functionality to retrieve calendar events from SOCS systems, which are commonly used by educational institutions in the UK. It handles the API limitations by implementing recursive fetching to ensure all events within a date range are retrieved.

## Features

- **Recursive Event Fetching**: Automatically handles API size limits by fetching events in chunks
- **Event Deduplication**: Removes duplicate events based on event ID
- **Event Sorting**: Sorts events chronologically by start time
- **XML Parsing**: Parses SOCS XML responses into structured Rust data types
- **Async Support**: Built with async/await for efficient network operations
- **Hidden API Parameters**: Includes access to unpublished and internal events via the `IncludeUnpublished` parameter

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
socs_calendar_parser = "0.1.0"
```

## Usage

### Basic Event Fetching

```rust
use chrono::NaiveDate;
use socs_calendar_parser::fetch_events_recursive;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let base_url = "https://your-socs-instance.com/api/calendar";
    let start_date = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
    let end_date = NaiveDate::from_ymd_opt(2025, 12, 31).unwrap();

    let events = fetch_events_recursive(base_url, start_date, end_date).await?;

    for event in events {
        println!("{}: {}", event.title, event.start);
    }

    Ok(())
}
```

### Working with Individual Events

```rust
use socs_calendar_parser::{fetch_calendar, parse_calendar_xml};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let base_url = "https://your-socs-instance.com/api/calendar";
    let start_date = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
    let end_date = NaiveDate::from_ymd_opt(2025, 1, 31).unwrap();

    // Fetch raw XML data
    let xml_data = fetch_calendar(base_url, start_date, end_date).await?;

    // Parse into structured events
    let events = parse_calendar_xml(xml_data)?;

    for event in events {
        if let Some(description) = &event.description {
            println!("Event: {}\nDescription: {}\nLocation: {}\n",
                    event.title, description, event.location);
        }
    }

    Ok(())
}
```

## API Details

### Hidden IncludeUnpublished Parameter

The SOCS API includes a hidden parameter `IncludeUnpublished=1` that provides access to unpublished events in the calendar. This parameter is not documented in the official SOCS API but allows retrieval of events that are not publicly visible through standard calendar interfaces.

**Important Notes:**
- This parameter may require special permissions on some SOCS installations
- Use with caution as it may expose draft or internal events
- The parameter is set to `1` (enabled) by default in this library

### Other Query Parameters

The library also sets the following parameters in API requests:
- `Sport=0`: Include sports-related events
- `CoCurricular=0`: Include co-curricular activities
- `IncludeInternal=1`: Include internal events

## Data Structures

### CalendarEvent

Represents a parsed calendar event:

```rust
pub struct CalendarEvent {
    pub event_id: String,
    pub title: String,
    pub description: Option<String>,
    pub location: String,
    pub categories: Vec<String>,
    pub start: EventTime,
    pub end: EventTime,
}
```

### EventTime

Represents event timing (either all-day or specific time):

```rust
pub enum EventTime {
    AllDay(NaiveDate),
    Specific { date: NaiveDate, time: NaiveTime },
}
```


## Dependencies

- `reqwest`: HTTP client for API requests
- `serde-xml-rs`: XML parsing
- `chrono`: Date and time handling
- `anyhow`: Error handling
- `urlencoding`: URL parameter encoding

## Development

### Building

```bash
cargo build
```

### Testing

```bash
cargo test
```

### Documentation

```bash
cargo doc --open
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Disclaimer

This library is not officially affiliated with SOCS or Capita. Use at your own risk and ensure compliance with your institution's terms of service when accessing calendar data.
