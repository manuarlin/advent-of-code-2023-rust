use std::time::Instant;

use crate::day_8::day_8::day_8;

pub mod utils;
pub mod day_1;
pub mod day_2;
pub mod day_3;
pub mod day_4;
pub mod day_5;
pub mod day_6;
pub mod day_7;
pub mod day_8;

fn main() {
    let start = Instant::now();
    day_8();
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
}





