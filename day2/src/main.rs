use utils;
use counter;
use itertools::izip;


fn part1() {
    let ids = utils::lines_from_file(String::from("day2.txt")).expect("Unable to load results from file");

    let mut twos = 0;
    let mut threes = 0;

    for id in &ids {
        let counts = id.chars().collect::<counter::Counter<_>>();
        let count_of_counts = counts.values().collect::<counter::Counter<_>>();

        if count_of_counts.contains_key(&2)  {
            twos += 1;
        }

        if count_of_counts.contains_key(&3)  {
            threes += 1;
        }
    }

    println!("{}", twos * threes);
}


fn part2() {
    let ids = utils::lines_from_file(String::from("day2.txt")).expect("Unable to load results from file");

    for outer_counter in 0..ids.len() {
        for inner_counter in (outer_counter + 1)..ids.len() {
            let mut diff_counter = 0;

            // TODO pull the results of the zipping into a vec of tuples
            
            for (first, second) in izip!(ids[outer_counter].chars(), ids[inner_counter].chars()) {
                if first != second {
                    diff_counter += 1;
                }
            }

            if diff_counter == 1 {
                let mut results = String::new();

                for (first, second) in izip!(ids[outer_counter].chars(), ids[inner_counter].chars()) {
                    if first == second {
                        results.push(first);
                    }
                }                

                println!("{}", results);
                return
            }
        }
    }
}


fn main() {
    part1(); 
    part2();
}