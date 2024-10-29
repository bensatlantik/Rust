// tests/lib.rs
use human_duration::format_duration;
use std::time::Duration;

#[test]
fn test_format_duration() {
    assert_eq!(format_duration(Duration::new(61, 0)), "1 minute, 1 second");
}
