use std::time::Instant;

use crate::day_7::day_7::day_7;

pub mod utils;
pub mod day_1;
pub mod day_2;
pub mod day_3;
pub mod day_4;
pub mod day_5;
pub mod day_6;
pub mod day_7;

fn main() {
    let start = Instant::now();
    day_7();
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
}





