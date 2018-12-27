use regex::Regex;
use std::{
    collections::{BTreeMap, BTreeSet},
    cmp::Ordering
};


fn part1() {
    let steps = utils::lines_from_file(String::from("day7.txt")).expect("Unable to load results from file");

    let matcher = Regex::new(r"Step ([A-Z]) must be finished before step ([A-Z]) can begin.").expect("Unable to create regex");

    let mut requirements: BTreeMap<char, BTreeSet<char>> = BTreeMap::new();
    let mut next_steps: BTreeMap<char, BTreeSet<char>> = BTreeMap::new();

    let mut starts: BTreeSet<char> = BTreeSet::new();
    let mut finishes: BTreeSet<char> = BTreeSet::new();

    for step in &steps {
        let captures = matcher.captures(step).unwrap();
        let before = captures.get(1).unwrap().as_str().chars().next().unwrap();
        let after = captures.get(2).unwrap().as_str().chars().next().unwrap();

        starts.insert(before);
        finishes.insert(after);

        requirements.entry(after).or_insert_with(BTreeSet::<char>::new).insert(before);
        next_steps.entry(before).or_insert_with(BTreeSet::<char>::new).insert(after);
    }

    // First, find the starting step, the first nodes in the tree.
    let mut candidates = starts.difference(&finishes).cloned().collect::<BTreeSet<_>>();
    let mut completed: BTreeSet<char> = BTreeSet::new();
    let mut final_output = String::new();

    while !candidates.is_empty() {
        let mut valid_node = ' ';

        for candidate_node in &candidates {
            if !requirements.contains_key(candidate_node) || requirements[candidate_node].difference(&completed).cloned().collect::<BTreeSet<_>>().is_empty() {
                // All requirements have been met for this node (or it doesn't have any), so it's safe to add the next steps
                valid_node = *candidate_node;
                break;
            }
        }

        if valid_node != ' ' {
            if next_steps.contains_key(&valid_node) {
                candidates = candidates.union(&next_steps[&valid_node]).cloned().collect();
            }

            completed.insert(valid_node);
            final_output.push(valid_node);

            candidates.remove(&valid_node);
        }
    }

    println!("{}", final_output);
}


#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct WorkItem {
    time_completed: u32,
    work_unit: char
}


impl WorkItem {

    fn new(current_time: u32, work_unit: char) -> Self {
        const MINIMUM_TIME: u32 = 60;

        WorkItem {
            time_completed: (current_time + (work_unit as u32) - ('A' as u32) + 1 + MINIMUM_TIME),
            work_unit: work_unit
        }
    }

}

impl Ord for WorkItem {
    fn cmp(&self, other: &WorkItem) -> Ordering {
        let mut result = self.time_completed.cmp(&other.time_completed);

        if result == Ordering::Equal {
            result = self.work_unit.cmp(&other.work_unit);
        }

         result
    }
}


impl PartialOrd for WorkItem {
    fn partial_cmp(&self, other: &WorkItem) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


fn part2() {
    const MAX_WORKERS: usize = 5;

    let steps = utils::lines_from_file(String::from("day7.txt")).expect("Unable to load results from file");

    let matcher = Regex::new(r"Step ([A-Z]) must be finished before step ([A-Z]) can begin.").expect("Unable to create regex");

    let mut requirements: BTreeMap<char, BTreeSet<char>> = BTreeMap::new();
    let mut next_steps: BTreeMap<char, BTreeSet<char>> = BTreeMap::new();

    let mut starts: BTreeSet<char> = BTreeSet::new();
    let mut finishes: BTreeSet<char> = BTreeSet::new();

    for step in &steps {
        let captures = matcher.captures(step).unwrap();
        let before = captures.get(1).unwrap().as_str().chars().next().unwrap();
        let after = captures.get(2).unwrap().as_str().chars().next().unwrap();

        starts.insert(before);
        finishes.insert(after);

        requirements.entry(after).or_insert_with(BTreeSet::<char>::new).insert(before);
        next_steps.entry(before).or_insert_with(BTreeSet::<char>::new).insert(after);
    }

    // For each of the start elements, put them into a work queue. Note that this assumes that there are not more elements to start with than can
    // fit into the queue itself
    let mut candidates = Vec::new();
    let mut work_queue: Vec<WorkItem> = Vec::new();

    for candidate in starts.difference(&finishes) {
        work_queue.push(WorkItem::new(0, *candidate));
    }

    work_queue.sort();

    let mut completed: BTreeSet<char> = BTreeSet::new();
    let mut final_output = String::new();

    while !work_queue.is_empty() {
        // Work out the next completed items (there might be more than one finishing at the same time)
        let time_completed = work_queue[0].time_completed;

        let (finished_work_units, mut new_queue): (Vec<WorkItem>, Vec<WorkItem>) = work_queue.drain(..).partition(|item| item.time_completed == time_completed);

        for finished_work_unit in &finished_work_units {
            let completed_work_unit = finished_work_unit.work_unit;

            completed.insert(completed_work_unit);
            final_output.push(completed_work_unit);

            if next_steps.contains_key(&completed_work_unit) {
                for next_work_step in &next_steps[&completed_work_unit] {
                    if !requirements.contains_key(&next_work_step) || requirements[&next_work_step].difference(&completed).cloned().collect::<BTreeSet<_>>().is_empty() {
                        // All requirements have been met for this node (or it doesn't have any), so it's safe to add it if possible
                        candidates.push(next_work_step);
                    }
                }
            }
        }

        candidates.sort(); 

        if candidates.is_empty() && new_queue.is_empty() {
            println!("Completed time: {}", time_completed);
            break;
        }

        while new_queue.len() < MAX_WORKERS && !candidates.is_empty() {
            let next_work_unit = candidates.remove(0);
            new_queue.push(WorkItem::new(time_completed, *next_work_unit));
        }

        new_queue.sort();

        work_queue = new_queue;
    }
}


fn main() {
    part1();
    part2();
}
