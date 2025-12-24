use anyhow::{Context, Result};
use chrono::NaiveDate;

/// Fetch calendar data from the SOCS API
pub async fn fetch_calendar(
    base_url: &str,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<String> {
    // Format dates as "DD MMM YY" (e.g., "10 Dec 25")
    let start_str = format_date_for_api(start_date);
    let end_str = format_date_for_api(end_date);

    // Build the URL with query parameters
    let url = format!(
        "{}&startdate={}&enddate={}&Sport=0&CoCurricular=0&IncludeInternal=1&IncludeUnpublished=1",
        base_url,
        urlencoding::encode(&start_str),
        urlencoding::encode(&end_str)
    );

    println!("Fetching calendar from: {}", url);

    // Fetch the data
    let response = reqwest::get(&url)
        .await
        .context("Failed to fetch calendar data")?;

    let status = response.status();
    if !status.is_success() {
        anyhow::bail!("HTTP request failed with status: {}", status);
    }

    let body = response
        .text()
        .await
        .context("Failed to read response body")?;

    Ok(body)
}

/// Format a date for the SOCS API in "DD MMM YY" format (e.g., "10 Dec 25")
fn format_date_for_api(date: NaiveDate) -> String {
    date.format("%d %b %y").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_date_for_api() {
        let date = NaiveDate::from_ymd_opt(2025, 12, 10).unwrap();
        let formatted = format_date_for_api(date);
        assert_eq!(formatted, "10 Dec 25");
    }
}
