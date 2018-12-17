use std::collections::{HashMap, BTreeMap};
use chrono::prelude::*;
use regex::Regex;
use counter;


fn get_timetable() -> HashMap<u32, Vec<u32>> {
    let mut inputs = utils::lines_from_file(String::from("day4.txt")).expect("Unable to load results from file");

    inputs.sort();

    let guard_matcher = Regex::new(r"^\[(\d{4}-\d{2}-\d{2} \d{2}:\d{2})\] Guard #(\d+) begins shift$").expect("Unable to create guard regex");
    let wake_matcher = Regex::new(r"^\[(\d{4}-\d{2}-\d{2} \d{2}:\d{2})\] wakes up$").expect("Unable to create wake regex");
    let sleep_matcher = Regex::new(r"^\[(\d{4}-\d{2}-\d{2} \d{2}:\d{2})\] falls asleep$").expect("Unable to create sleep regex");
 
    let mut timetable: HashMap<u32, Vec<u32>> = HashMap::new();

    let mut id = 0;
    let mut sleep_minute = 0;

    for input in &inputs {
        if input.contains("Guard") {
            let capture = guard_matcher.captures(input).expect("Unable to apply guard regex");
            id = capture[2].parse::<u32>().unwrap();

            timetable.entry(id).or_insert_with(Vec::new);
        } else if input.contains("sleep") {
            let capture = sleep_matcher.captures(input).expect("Unable to apply sleep regex");
            let sleep_date_time = Utc.datetime_from_str(&capture[1], "%Y-%m-%d %H:%M").expect("Unable to parse sleep datetime properly"); 

            sleep_minute = sleep_date_time.minute();
        } else {
            let capture = wake_matcher.captures(input).expect("Unable to apply wake regex");
            let wake_date_time = Utc.datetime_from_str(&capture[1], "%Y-%m-%d %H:%M").expect("Unable to parse wake datetime properly"); 
            let wake_minute = wake_date_time.minute();

            for minute in sleep_minute..wake_minute {
                timetable.entry(id).and_modify(|e| e.push(minute));
            }
        }
    }

    timetable
}


fn part1() {
    let timetable = get_timetable();

    // Get the count of number of minutes per guard.
    let total_minute_map = timetable.iter().map(|(k, v)| (v.len() as u32, *k)).collect::<BTreeMap<u32, u32>>();

    // As we've put this into a BTreeMap, and keyed this by the number of minutes, picking the last iterator will give us a value for the guard
    // who's been asleep the most.
    let guard_id = total_minute_map.iter().rev().next().unwrap().1;

    // Now pull out that guard's minutes, and pick the most common minute for xe to be asleep.
    let minutes = &timetable[guard_id];
    let minute_counter_minutes = minutes.iter().collect::<counter::Counter<_>>().most_common();
    let most_common_minute = minute_counter_minutes.iter().next().unwrap().0;

    print!("{:?}\n", *most_common_minute * *guard_id);
}


fn part2() {
    let timetable = get_timetable();

    // Build up a map of guard id -> counter of minutes
    let guard_minute_count_map = timetable.iter().map(|(k, v)| (*k, counter::Counter::init(v.iter()))).collect::<HashMap<_, counter::Counter<_>>>();

    // Now create a map of guard id -> most common minutes list
    let guard_most_common_minute_map = guard_minute_count_map.iter().map(|(k, v)| (*k, v.most_common())).collect::<HashMap<_, _>>();

    // Finally, build up a BTreeMap of highest minute frequency -> guard id
    let guard_most_frequent_minute_map = guard_most_common_minute_map.iter().filter_map(|(k, v)| match v.len() {
        0 => None,
        _ => Some((v[0].1, *k))
    }).collect::<BTreeMap<_, _>>();

    let guard_id = guard_most_frequent_minute_map.iter().rev().next().unwrap().1;

    // Get the max minute frequency element (the last in the BTreeMap), then get the minute number itself
    let minute_number = *guard_most_common_minute_map[guard_id][0].0;

    println!("{}", guard_id * minute_number);
}

fn main() {
    part1();
    part2();
}
