//! Time utilities

use chrono::{DateTime, Utc};
use std::time::{Duration, Instant};

/// Utility functions for time operations
pub struct TimeUtils;

impl TimeUtils {
    /// Get current UTC timestamp
    pub fn now() -> DateTime<Utc> {
        Utc::now()
    }

    /// Format duration in human-readable form
    pub fn format_duration(duration: Duration) -> String {
        let secs = duration.as_secs();
        
        if secs < 1 {
            format!("{:.2}ms", duration.as_millis() as f64)
        } else if secs < 60 {
            format!("{:.1}s", secs as f64)
        } else if secs < 3600 {
            format!("{}m {}s", secs / 60, secs % 60)
        } else {
            format!("{}h {}m", secs / 3600, (secs % 3600) / 60)
        }
    }

    /// Measure execution time of a function
    pub fn measure<T, F: FnOnce() -> T>(f: F) -> (T, Duration) {
        let start = Instant::now();
        let result = f();
        let elapsed = start.elapsed();
        (result, elapsed)
    }

    /// Calculate ETA based on progress
    pub fn calculate_eta(elapsed: Duration, progress: f64, total: f64) -> Option<Duration> {
        if progress <= 0.0 || total <= 0.0 {
            return None;
        }
        
        let rate = progress / elapsed.as_secs_f64();
        let remaining = total - progress;
        
        Some(Duration::from_secs_f64(remaining / rate))
    }

    /// Sleep asynchronously
    pub async fn sleep_async(duration: Duration) {
        tokio::time::sleep(duration).await;
    }

    /// Create a timeout future
    pub fn timeout<T, F>(duration: Duration, future: F) -> tokio::time::Timeout<F>
    where
        F: std::future::Future,
    {
        tokio::time::timeout(duration, future)
    }

    /// Convert timestamp to ISO 8601 string
    pub fn to_iso8601(dt: DateTime<Utc>) -> String {
        dt.to_rfc3339()
    }

    /// Parse ISO 8601 string to DateTime
    pub fn from_iso8601(s: &str) -> Option<DateTime<Utc>> {
        DateTime::parse_from_rfc3339(s)
            .ok()
            .map(|dt| dt.with_timezone(&Utc))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_duration() {
        assert!(TimeUtils::format_duration(Duration::from_millis(500)).contains("ms"));
        assert!(TimeUtils::format_duration(Duration::from_secs(30)).contains("s"));
        assert!(TimeUtils::format_duration(Duration::from_secs(90)).contains("m"));
    }

    #[test]
    fn test_measure() {
        let (_, elapsed) = TimeUtils::measure(|| {
            std::thread::sleep(Duration::from_millis(10));
        });
        assert!(elapsed >= Duration::from_millis(10));
    }
}
