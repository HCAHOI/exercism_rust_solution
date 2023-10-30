use time::PrimitiveDateTime as DateTime;
use std::time::Duration;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    start + Duration::new(1_000_000_000, 0)
}