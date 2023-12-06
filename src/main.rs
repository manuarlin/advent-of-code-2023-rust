use std::time::Instant;

use crate::day_6::day_6::day_6;

pub mod utils;
pub mod day_1;
pub mod day_2;
pub mod day_3;
pub mod day_4;
pub mod day_5;
pub mod day_6;

fn main() {
    let start = Instant::now();
    day_6();
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
}





