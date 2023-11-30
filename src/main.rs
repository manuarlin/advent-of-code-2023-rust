use crate::utils::read_file;

pub mod utils;

fn main() {
    let lines = read_file::read_lines("day1-part1");

    lines.iter()
        .for_each(|line| println!("{}", line))
}



