fn implementation(recipe_count: usize, next_recipe_count: usize) {
    let mut recipe_scoreboard: Vec<usize> = Vec::new();
    // let mut recipe_scoreboard =
    //     String::with_capacity(recipe_count as usize + next_recipe_count as usize);

    // recipe_scoreboard.push_str("37");

    recipe_scoreboard.push(3);
    recipe_scoreboard.push(7);

    let mut elf_1_index: usize = 0;
    let mut elf_2_index: usize = 1;

    let target_string = recipe_count.to_string();
    let mut iteration_count = recipe_scoreboard.len();

    loop {
        let elf_1_number = recipe_scoreboard[elf_1_index];
        let elf_2_number = recipe_scoreboard[elf_2_index];

        let new_recipe_number = elf_1_number + elf_2_number;
        let new_recipe_string = new_recipe_number.to_string();

        let new_recipe_digits: Vec<usize> = new_recipe_string.chars().map(|c| c.to_digit(10).unwrap() as usize).collect();
        recipe_scoreboard.extend(&new_recipe_digits);

        elf_1_index = (elf_1_index + 1 + elf_1_number) % recipe_scoreboard.len();
        elf_2_index = (elf_2_index + 1 + elf_2_number) % recipe_scoreboard.len();

        // This is pretty fast in Rust, and it seems that the first occurrence doesn't happen until way after the answer for part 1. So we'll
        // add a couple of hundred million iterations to the loop.
        if iteration_count > (recipe_count + next_recipe_count + 200_000_000) {
            break;
        }

        iteration_count += 1;
    }

    let final_string: String = recipe_scoreboard.iter().map(|c| c.to_string()).collect::<String>();
    let final_slice_index = recipe_count  + next_recipe_count;

    println!(
        "Part 1: {}, Part 2: {:?}",
        final_string[(recipe_count as usize)..(final_slice_index as usize)].to_string(),
        final_string.find(&target_string)
    );
}

fn main() {
    implementation(894_501, 10);
}
