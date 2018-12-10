use simple_matrix::Matrix;
use regex::Regex;
use std::collections::HashSet;


fn part1() {
    let squares = utils::lines_from_file(String::from("day3.txt")).expect("Unable to load results from file");

    let mut matrix: Matrix<i32> = Matrix::new(1100, 1100);

    let matcher = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").expect("Unable to create regex");

    for square in &squares {
        for capture in matcher.captures_iter(square) {
            let start_left = capture.get(2).unwrap().as_str().parse::<usize>().unwrap();
            let start_down = capture.get(3).unwrap().as_str().parse::<usize>().unwrap();
            let width = capture.get(4).unwrap().as_str().parse::<usize>().unwrap();
            let height = capture.get(5).unwrap().as_str().parse::<usize>().unwrap();

            for x in start_left..(start_left + width) {
                for y in start_down..(start_down + height) {
                    let current = matrix.get(x, y).expect("Unable to read current matrix point");
                    matrix.set(x, y, current + 1);
                }
            }
        }
    }

    let mut sum = 0;

     matrix.apply(|n| if *n > 1 {sum += 1});

    println!("{}", sum);
}


fn part2() {
    let squares = utils::lines_from_file(String::from("day3.txt")).expect("Unable to load results from file");

    let mut matrix: Matrix<i32> = Matrix::new(1100, 1100);

    let matcher = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").expect("Unable to create regex");

    let mut ids: HashSet<i32> = HashSet::new();

    for square in &squares {
        for capture in matcher.captures_iter(square) {
            let mut no_collision = true;

            let id = capture.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let start_left = capture.get(2).unwrap().as_str().parse::<usize>().unwrap();
            let start_down = capture.get(3).unwrap().as_str().parse::<usize>().unwrap();
            let width = capture.get(4).unwrap().as_str().parse::<usize>().unwrap();
            let height = capture.get(5).unwrap().as_str().parse::<usize>().unwrap();

            for x in start_left..(start_left + width) {
                for y in start_down..(start_down + height) {
                    let current = matrix.get(x, y).expect("Unable to get current matrix point");

                    if *current > 0 {
                        ids.remove(&current);
                        no_collision = false;
                    }

                    matrix.set(x, y, id);
                }
            }

            if no_collision {
                ids.insert(id);
            }
        }
    }

    println!("{:?}", ids);
}


fn main() {
    part1();
    part2();
}
