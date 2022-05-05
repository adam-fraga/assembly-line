// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

/*
    Speed of the assembly line available
         - 1 to 10
    Cars production increases linearly:
         - 2 -> 242, 3 -> 484...
     Success rate decrease with line improvement:
         - `1` to `4`: 100% success rate.
         - `5` to `8`: 90% success rate.
         - `9` and `10`: 77% success rate.
*/

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let success_rate: f64 = match speed {
        1 | 2 | 3 | 4 => 0.0,
        5 | 6 | 7 | 8 => 0.9,
        9 | 10 => 0.77,
        other => return 0 as f64,
    };
    let cars_production: i32 = 221 * speed as i32;
    if success_rate != 0.0 {
        success_rate * cars_production as f64
    } else {
        cars_production as f64
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let result: f64 = production_rate_per_hour(speed) / 60 as f64;
    result as u32
}
