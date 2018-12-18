use simple_matrix::Matrix;
use std::collections::{HashMap, HashSet, BTreeMap};
use counter;


fn get_coordinate_map() -> HashMap<u32, (i32, i32)> {
    let coordinates = utils::lines_from_file(String::from("day6.txt")).expect("Unable to load results from file");

    let mut results: HashMap<u32, (i32, i32)> = HashMap::new();

    for (id, val) in coordinates.iter().enumerate() {
        let mut coordinate_strings = val.split(", ");
        let x = coordinate_strings.next().unwrap().parse::<i32>().unwrap();
        let y = coordinate_strings.next().unwrap().parse::<i32>().unwrap();

        // We'll make IDs start at 1, 0 will be a sentinel value for no ID found
        results.insert((id + 1) as u32, (x, y));
    }

    results
}


fn part1() {
    let coordinates = get_coordinate_map();

    let x_coords = coordinates.iter().map(|(_, v)| v.0).collect::<Vec<i32>>();
    let y_coords = coordinates.iter().map(|(_, v)| v.1).collect::<Vec<i32>>();

    let max_x = *x_coords.iter().max().unwrap();
    let max_y = *y_coords.iter().max().unwrap();

    let mut surface: Matrix<u32> = Matrix::new((max_x + 1) as usize, (max_y + 1) as usize);

    // Build up a set of the IDs that will be excluded from analysis
    let mut excluded_ids: HashSet<u32> = HashSet::new();

    excluded_ids.insert(0);

    for x_coord in 0..surface.cols() {
        for y_coord in 0..surface.rows() {
            let mut distance_id_map: BTreeMap<i32, Vec<u32>> = BTreeMap::new();

            for (id, (x, y)) in &coordinates {
                let new_distance = (*x as i32 - x_coord as i32).abs() + (*y  as i32 - y_coord as i32).abs();

                distance_id_map.entry(new_distance).or_insert_with(Vec::new).push(*id);
            }

            let (_, closest_ids) = distance_id_map.iter().next().unwrap();

            surface.set(x_coord as usize, y_coord as usize, match closest_ids.len() {
                1 => closest_ids[0],
                _ => 0,
            });

            if x_coord == 0 || y_coord == 0 || x_coord == surface.rows() - 1 || y_coord == surface.cols() - 1 {
                excluded_ids.extend(closest_ids.iter());
            }
        }
    }

    let id_counts = surface.iter().filter(|id| !excluded_ids.contains(id)).collect::<counter::Counter<_>>().most_common();

    println!("{:?}", id_counts);
}


fn part2() {
    const MAX_DISTANCE: i32 = 10_000;

    let coordinates = get_coordinate_map();

    let x_coords = coordinates.iter().map(|(_, v)| v.0).collect::<Vec<i32>>();
    let y_coords = coordinates.iter().map(|(_, v)| v.1).collect::<Vec<i32>>();

    let max_x = *x_coords.iter().max().unwrap();
    let max_y = *y_coords.iter().max().unwrap();

    let mut region_count = 0;

    for x_coord in 0..=max_x {
        for y_coord in 0..=max_y {
            let mut total_distance = 0i32;

            for (x, y) in coordinates.values() {
                let new_distance = (*x as i32 - x_coord as i32).abs() + (*y  as i32 - y_coord as i32).abs();

                total_distance += new_distance;
            }

            if total_distance < MAX_DISTANCE {
                region_count += 1;
            }
        }
    }

    println!("{}", region_count);
}


fn main() {
    part1();
    part2();
}
