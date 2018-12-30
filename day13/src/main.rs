use simple_matrix::Matrix;
use std::{
    cmp::Ordering,
    collections::{HashSet, HashMap}
};
use uuid::Uuid;


#[derive(Debug, Eq, PartialEq)]
struct Cart {
    id: Uuid,
    x: i32,
    y: i32,
    x_velocity: i32,
    y_velocity: i32,
    turn_state: i32,
}


impl Cart {
    fn new(x: usize, y: usize, x_velocity: i32, y_velocity: i32) -> Self {
        let id = Uuid::new_v4();

        Cart {
            id,
            x: x as i32,
            y: y as i32,
            x_velocity,
            y_velocity,
            turn_state: 0,
        }
    }

    fn move_forward(&mut self) {
        self.x += self.x_velocity;
        self.y += self.y_velocity;
    }

    fn turn_left(&mut self) {
        if self.y_velocity == 0 {
            self.y_velocity = -self.x_velocity;
            self.x_velocity = 0;
        } else {
            self.x_velocity = self.y_velocity;
            self.y_velocity = 0;
        }
    }

    fn turn_right(&mut self) {
        if self.y_velocity == 0 {
            self.y_velocity = self.x_velocity;
            self.x_velocity = 0;
        } else {
            self.x_velocity = -self.y_velocity;
            self.y_velocity = 0;
        }
    }

    fn process_intersection(&mut self) {
        if self.turn_state == 0 {
            self.turn_left();            
        } else if self.turn_state == 2 {
            self.turn_right();
        }

        self.turn_state = (self.turn_state + 1) % 3;
    }
}


impl Ord for Cart {
    fn cmp(&self, other: &Cart) -> Ordering {
        let mut result = self.x.cmp(&other.x);

        if result == Ordering::Equal {
            result = self.y.cmp(&other.y);
        }

         result
    }
}


impl PartialOrd for Cart {
    fn partial_cmp(&self, other: &Cart) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


struct Track {
    grid: Matrix<char>,
    carts: Vec<Cart>
}


fn load_track() -> Track {
    let inputs = utils::lines_from_file(String::from("day13.txt")).expect("Unable to load inputs from file");

    let row_length = inputs.iter().next().unwrap().len();
    let row_count = inputs.len();

    let mut grid: Matrix<char> = Matrix::new(row_count, row_length);
    let mut carts: Vec<Cart> = Vec::new();

    for (row_number, row) in inputs.iter().enumerate() {
        for (column_number, cell) in row.chars().enumerate() {
            let cell_value = match cell {
                '|' | '-' | '/' | '\\' | '+' | ' ' => cell,
                '<' | '>' => '-',
                'v' | '^' => '|',
                _ => panic!("Unknown cell input")
            };

            grid.set(row_number, column_number, cell_value);

            if cell == '^' || cell == 'v' || cell == '<' || cell == '>' {
                let cart = match cell {
                    '^' => Cart::new(column_number, row_number, 0, -1),
                    'v' => Cart::new(column_number, row_number, 0, 1),
                    '<' => Cart::new(column_number, row_number, -1, 0),
                    '>' => Cart::new(column_number, row_number, 1, 0),
                    _ => panic!("Unknown cart input")
                };

                carts.push(cart);
            }
        }
    }

    Track {
        grid,
        carts
    }
}


fn implementation(remove_crashed_carts: bool) {
    let mut track = load_track();

    let mut carts_to_ignore: HashSet<Uuid> = HashSet::new();

    loop {
        track.carts.sort();

        let mut coordinates: HashMap<(i32, i32), HashSet<Uuid>> = HashMap::new();

        for cart in &track.carts {
            if !carts_to_ignore.contains(&cart.id) {
                coordinates.entry((cart.x, cart.y)).or_insert_with(HashSet::<Uuid>::new).insert(cart.id);
            }
        }

        if remove_crashed_carts && coordinates.len() == 1 {
            let final_coordinate = coordinates.keys().next().unwrap();

            println!("Final coordinate: {},{}", final_coordinate.0, final_coordinate.1);
            return;
        }

        for cart in &mut track.carts {
            if carts_to_ignore.contains(&cart.id) {
                continue;
            }

            let old_coordinates = (cart.x, cart.y);

            cart.move_forward();

            let mut crashed = false;

            for (other_coordinates, ids) in &coordinates {
                if *other_coordinates == (cart.x, cart.y) && !ids.is_empty() && !ids.contains(&cart.id) {
                    // Looks like we've got a collision!
                    if !remove_crashed_carts {
                        println!("Collision at {}, {}", cart.x, cart.y);
                        return;                    
                    } else {
                        carts_to_ignore.insert(cart.id);
                        carts_to_ignore.extend(ids);
                        crashed = true;
                        break;
                    }
                }
            }

            if crashed {
                continue;
            }

            // Update coordinate map
            coordinates.entry(old_coordinates).or_insert_with(HashSet::<Uuid>::new).remove(&cart.id);
            coordinates.entry((cart.x, cart.y)).or_insert_with(HashSet::<Uuid>::new).insert(cart.id);

            let &new_cell = &track.grid.get(cart.y as usize, cart.x as usize).unwrap();

            if *new_cell == '/' {
                if cart.x_velocity == 0 {
                    cart.turn_right();
                } else {
                    cart.turn_left();
                }
            } else if *new_cell == '\\' {
                if cart.y_velocity == 0 {
                    cart.turn_right();
                } else {
                    cart.turn_left();
                }
            } else if *new_cell == '+' {
                cart.process_intersection();
            }
        }
    }
}


fn main() {
    implementation(false);
    implementation(true);
}

