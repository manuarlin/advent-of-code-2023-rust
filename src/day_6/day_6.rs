use crate::utils::read_file;

pub fn day_6() {
    let lines = read_file::read_lines("day6");
    let times_line: Vec<&str> = lines[0].split("Time:").collect();
    let times_line: &str = times_line[1].trim();

    let distances_line: Vec<&str> = lines[1].split("Distance:").collect();
    let distances_line: &str = distances_line[1].trim();

    let distance: Vec<&str> = distances_line.split_whitespace().collect();
    let distance: i64 = distance.join("").parse::<i64>().unwrap();
    let time: Vec<&str> = times_line.split_whitespace().collect();
    let time: i64 = time.join("").parse::<i64>().unwrap();
    let race = Race::new(time, distance);

    let margin = race.get_possible_charging_times().len();

    println!("{margin:?}")

}

#[derive(Debug)]
struct Race {
    time: i64,
    record: i64,
}


impl Race {
    pub fn new(time: i64, record: i64) -> Self {
        Self { time, record }
    }

    fn get_possible_charging_times(&self) -> Vec<i64> {
        let mut possibilities: Vec<i64>= vec!();

        for i in 0..=self.time {
            let speed = i;
            let distance_ran = (self.time - i) * speed;

            if distance_ran > self.record {
                possibilities.push(i)
            } else {
                if !possibilities.is_empty() {
                    break
                }
            }
        }
        possibilities
    }
}