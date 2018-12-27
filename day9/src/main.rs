use std::collections::{BTreeMap, VecDeque};

fn implementation(player_count: i32, max_marble: i32) {
    let mut buffer: VecDeque<i32> = VecDeque::with_capacity(max_marble as usize);

    buffer.push_back(0);

    let mut current_index = 0;

    let mut scores: BTreeMap<i32, u64> = BTreeMap::new();

    let mut counter = 0;

    for marble_number in 1..=max_marble {
        if marble_number % (max_marble / 100) == 0 {
            counter += 1;
            println!("Tick {}", counter);
        }


        if marble_number % 23 != 0 {
            let new_index = (current_index + 2) % buffer.len();

            if new_index == 0 {
                buffer.push_back(marble_number);
                current_index = buffer.len() - 1;
            } else {
                buffer.insert(new_index, marble_number);
                current_index = new_index;
            }
        } else {
            let index_to_remove = if current_index < 7 {current_index + buffer.len() - 7} else {current_index - 7};
            let removed_element = buffer.remove(index_to_remove).unwrap();
            let new_score = (marble_number + removed_element) as u64;
            current_index = index_to_remove;

            *scores.entry(marble_number % player_count).or_insert(0) += new_score;
        }
    }

    println!("High score: {:?}", scores.values().max());
}

fn main() {
    // This one takes a while to run, it's not optimised at all, so make sure you use a Release build!
    // Use of a proper linked list would make this a lot faster, but Rust doesn't have one by default that allows insertions at
    // arbitrary points in the structure (it's missing iterators that would allow this kind of behaviour). We could potentially use something
    // like https://cglab.ca/~abeinges/blah/too-many-lists/book/infinity-double-single.html , although I'm just being lazy for now and letting it
    // run for a while while I do some chores and surf the web :)
    implementation(419, 71052);
    implementation(419, 71052 * 100);
}
