use utils::read_file;
use std::collections::HashSet;


fn reduce_polymer(file_contents: &String, tokens: &HashSet<String>) -> usize {
    let mut current_file_contents = file_contents.clone();

    loop {
        let current_length = current_file_contents.len();

        let mut new_file_contents = String::with_capacity(current_length);

        let mut buffer = String::with_capacity(2);

        for current_char in current_file_contents.chars() {
            match buffer.len() {
                0 => buffer.push(current_char),
                1 => buffer.push(current_char),
                _ => { if tokens.contains(&buffer) {
                        buffer.clear();
                    } else {
                        new_file_contents.push(buffer.remove(0));
                    }

                    buffer.push(current_char);
                }
            }
        }

        new_file_contents.push_str(&buffer);

        if new_file_contents.len() == current_length {
            return current_length;
        }

        current_file_contents = new_file_contents;
    }
}


fn part1() {
    let file_contents = read_file(String::from("day5.txt"));

    let mut tokens: HashSet<String> = HashSet::new();

    for letter in String::from("abcdefghijklmnopqrstuvwxyz").chars() {
        let mut token = String::with_capacity(2);

        token.push(letter);
        token.push(letter.to_uppercase().next().unwrap());

        tokens.insert(token.clone());
        tokens.insert(token.chars().rev().collect());
    }

    let result = reduce_polymer(&file_contents, &tokens);

    println!("{}", result);
}


fn part2() {
    let file_contents = read_file(String::from("day5.txt"));

    let mut tokens: HashSet<String> = HashSet::new();

    for letter in String::from("abcdefghijklmnopqrstuvwxyz").chars() {
        let mut token = String::with_capacity(2);

        token.push(letter);
        token.push(letter.to_uppercase().next().unwrap());

        tokens.insert(token.clone());
        tokens.insert(token.chars().rev().collect());
    }

    let mut results: Vec<usize> = Vec::new();

    for token in &tokens {
        let mut filtered_file_contents = String::with_capacity(file_contents.len());

        for letter in file_contents.chars() {
            if !token.contains(letter) {
                filtered_file_contents.push(letter);
            }
        }

        let result = reduce_polymer(&filtered_file_contents, &tokens);

        results.push(result);
    }

    println!("{:?}", results.iter().min());
}


fn main() {
    part1();
    part2();
}
