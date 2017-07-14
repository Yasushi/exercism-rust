extern crate chrono;
use chrono::*;
use std::ops::Add;

// Returns a UTC DateTime one billion seconds after start.
pub fn after(start: DateTime<UTC>) -> DateTime<UTC> {
    start.add(Duration::seconds(1_000_000_000))
}
