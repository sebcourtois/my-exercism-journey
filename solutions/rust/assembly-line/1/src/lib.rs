// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let full_rate = (speed as f64) * 221.0;
    let success_rate = if speed <= 4 { 1.0 } else if speed <= 8 { 0.9 } else { 0.77 };

    full_rate * success_rate
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0) as u32
}
