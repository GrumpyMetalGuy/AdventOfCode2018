use std::collections::HashSet;
use utils;


fn part1() {
    let mut total_frequency = 0;

    let frequencies = utils::lines_from_file(String::from("day1.txt")).expect("Unable to load results from file");

    for frequency in &frequencies {
        let current_frequency = frequency.parse::<i32>().expect("Unable to parse integer");

        total_frequency += current_frequency;
    }

    println!("{}", total_frequency)
}



fn part2() {
    let mut tracker_set: HashSet<i32> = HashSet::new();
    let mut total_frequency = 0;

    tracker_set.insert(total_frequency);

    let frequencies = utils::lines_from_file(String::from("day1.txt")).expect("Unable to load results from file");

    loop {
        for frequency in &frequencies {
            let current_frequency = frequency.parse::<i32>().expect("Unable to parse integer");

            total_frequency += current_frequency;

            if tracker_set.contains(&total_frequency) {
                // Found duplicate entry, print this and leave
                println!("{}", total_frequency);
                return
            }

            tracker_set.insert(total_frequency);
        }
    }
}


fn main() {
    part1();
    part2();
}
