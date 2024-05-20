use std::{ops::Add, time::Duration};

use time::PrimitiveDateTime as DateTime;

const GIGA: u64 = 1000_000_000; // Giga = thousand, million

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let giga_seconds = Duration::new(GIGA, 0);
    start.add(giga_seconds)
}
