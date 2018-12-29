use std::collections::{HashMap, VecDeque};

const NOPLANT: char = '.';
const PLANT: char = '#';

fn get_input() -> (String, HashMap<String, char>) {
    let inputs = utils::lines_from_file(String::from("day12.txt")).expect("Unable to load inputs from file");

    let initial_state_input = inputs.iter().next().unwrap();
    let initial_state_tokens: Vec<&str> = initial_state_input.split(' ').collect();

    let mut transitions: HashMap<String, char> = HashMap::new();

    for transition_input in &inputs {
        let transition_tokens: Vec<&str> = transition_input.split(" => ").collect();

        if transition_tokens.len() == 2 {
            let state = transition_tokens[0];
            let result = transition_tokens[1];

            transitions.insert(String::from(state), result.chars().next().unwrap());
        }
    }
    
    (String::from(initial_state_tokens[2]), transitions)
}


fn construct_initial_state_vector(initial_input: &str, padding: usize) -> VecDeque<char> {
    let mut state_vector: VecDeque<char> = VecDeque::new();

    for _ in 0..padding {
        state_vector.push_back(NOPLANT);
    }

    for input in initial_input.chars() {
        state_vector.push_back(input);
    }

    for _ in 0..padding {
        state_vector.push_back(NOPLANT);
    }

    state_vector
}


fn implementation(timesteps: usize) {
    let (initial_input, transitions) = get_input();

    // We now have our set of transitions. We will pick a starting offset to start as the centre position, make it big enough to cover the 
    // problem space.
    let offset = 10000;

    // We'll also determine how much padding is on either side of the centre of the transition
    let transition_size = transitions.keys().next().unwrap().len();
    let transition_centre = (transition_size - 1) / 2;

    let mut plant_vector = construct_initial_state_vector(&initial_input, offset);

    let mut previous_value = 0;

    for generation in 0..timesteps {
        let mut next_generation: VecDeque<char> = VecDeque::new();

        for _ in 0..transition_centre {
            next_generation.push_back(NOPLANT);
        }

        let mut final_value: i32 = 0;

        for bucket_offset in 0..=(plant_vector.len() - transition_size) {
            let mut grow_plant = false;

            for (transition, final_state) in &transitions {
                let mut transition_matched = true;

                for pot_number in 0..transition_size {
                    if plant_vector[bucket_offset + pot_number] != transition.chars().nth(pot_number).unwrap() {
                        transition_matched = false;
                    }
                }

                if transition_matched {
                    grow_plant = *final_state == PLANT;
                    break;
                }
            };

            if grow_plant {
                next_generation.push_back(PLANT);
                final_value += (bucket_offset - offset + transition_centre) as i32;
            } else {
                next_generation.push_back(NOPLANT);
            }
        }

        for _ in 0..transition_centre {
            next_generation.push_back(NOPLANT);
        }

        plant_vector = next_generation;

        println!("Generation: {}, value: {}, value difference: {}", generation + 1, final_value, final_value - previous_value);

        previous_value = final_value;
    }
}


fn main() {
    implementation(2000);
    
    // Values converge so that each generation adds 75, and after 2000 generations, the value is 151113.
    println!("50 billion generation answer: {}", (50_000_000_000i64 - 2000) * 75 + 151_113);
}
