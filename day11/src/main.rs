use simple_matrix::Matrix;


fn get_grid(grid_serial_number: i32) -> Matrix<i32> {
    let mut fuel_grid: Matrix<i32> = Matrix::new(300, 300);

    // Start by determining the value per fuel cell in the grid
    for col in 1..=fuel_grid.cols() {
        let rack_id = (col + 10) as i32;

        for row in 1..=fuel_grid.rows() {
            let mut power_level = rack_id * row as i32;
            power_level += grid_serial_number;
            power_level *= rack_id;

            if power_level < 100 {
                power_level = 0;
            } else {
                power_level = (power_level / 100) % 10;
            }

            power_level -= 5;

            fuel_grid.set(col - 1, row - 1, power_level);
        }
    }

    fuel_grid
}


fn part1(grid_serial_number: i32) {
    let fuel_grid = get_grid(grid_serial_number);

    let mut candidate_x = 0;
    let mut candidate_y = 0;
    let mut total_power = 0;

    const GRID_SIZE: usize = 3;

    for col in 0..(fuel_grid.cols() - GRID_SIZE) {
        for row in 0..(fuel_grid.rows() - GRID_SIZE) {
            let mut new_total_power = 0;

            for x_adj in 0..GRID_SIZE {
                for y_adj in 0..GRID_SIZE {
                    new_total_power += fuel_grid.get(col + x_adj, row + y_adj).unwrap();
                }
            }

            if new_total_power > total_power {
                total_power = new_total_power;
                candidate_x = col;
                candidate_y = row;
            }
        }
    }

    println!("x: {}, y: {}", candidate_x + 1, candidate_y + 1);
}


fn part2(grid_serial_number: i32) {
    let fuel_grid = get_grid(grid_serial_number);

    let mut candidate_x = 0;
    let mut candidate_y = 0;
    let mut candidate_grid_size = 0;
    let mut total_power = 0;

    for grid_size in 1..300 {
        for col in 0..(fuel_grid.cols() - grid_size) {
            for row in 0..(fuel_grid.rows() - grid_size) {
                let mut new_total_power = 0;

                for x_adj in 0..grid_size {
                    for y_adj in 0..grid_size {
                        new_total_power += fuel_grid.get(col + x_adj, row + y_adj).unwrap();
                    }
                }

                if new_total_power > total_power {
                    total_power = new_total_power;
                    candidate_x = col;
                    candidate_y = row;
                    candidate_grid_size = grid_size;
                }
            }
        }
    }

    println!("x: {}, y: {}, grid size: {}", candidate_x + 1, candidate_y + 1, candidate_grid_size);
}


fn main() {
    let grid_serial_number = 4842;

    part1(grid_serial_number);
    part2(grid_serial_number);
}
