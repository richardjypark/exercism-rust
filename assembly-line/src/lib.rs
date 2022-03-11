// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

const CARS_PER_HOUR: f64 = 221.0;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    match speed {
        speed if speed >= 1 && speed <= 4 => speed as f64 * CARS_PER_HOUR,
        speed if speed >= 5 && speed <= 8 => (0.9 * speed as f64) * CARS_PER_HOUR,
        speed if speed >= 9 && speed <= 10 => (0.77 * speed as f64) * CARS_PER_HOUR,
        _ => 0.0,
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0) as u32
}
