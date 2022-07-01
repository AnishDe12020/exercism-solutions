// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let success_rate = if speed >= 1 && speed <= 4 {
        1.0
    } else if speed >= 5 && speed <= 8 {
        0.9
    } else if speed == 9 || speed == 10 {
        0.77
    } else {
        1.0
    };

    return f64::from(speed) * 221.0 * success_rate;
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let prod_rate = production_rate_per_hour(speed);

    let per_minute = prod_rate / 60.0;

    return per_minute as u32;
}
