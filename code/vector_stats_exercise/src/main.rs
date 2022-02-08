// Solves the first example exercise at the end of Chapter 8 in the Rust Book:
//
// " Given a list of integers, use a vector and return the median
// (when sorted, the value in the middle position) and mode
// (the value that occurs most often; a hash map will be helpful here) of the list. "

use std::collections::BTreeMap; // for sorted maps
use rand::Rng;  // to create vector of random integers

const MIN: u32 = 1;     // lowest value possible in the list of integers
const MAX: u32 = 50;    // highest value possible in the list of integers
const SIZE: u32 = 100;  // number of elements in the list of integers

fn main() {
    let integer_list = create_random_vector(MIN,MAX,SIZE);
    println!("integer_list: {:?}", integer_list);
    println!();
    println!("median: {}", median(&integer_list));
    println!("mode: {}", mode(&integer_list));
}

fn create_random_vector(min_val: u32, max_val: u32, quantity: u32) -> Vec<u32> {
    let mut vector: Vec<u32> = Vec::new();
    for _ in 0..quantity {
        vector.push(
            rand::thread_rng().gen_range(min_val..=max_val)
        );
    }
    vector.sort();
    vector
}

// a lazy median function, does not properly find median of even-sized lists
// TOTO: fix median for even-sized lists
fn median(integer_list: &Vec<u32>) -> u32 {
    let mut list = integer_list.to_vec();   // makes a mutable copy of integer_list
    while list.len() > 1 {
        list.remove(0);
        if list.len() > 1 {
            list.pop();
        }
    }
    list[0] // returns only remaining element
}

fn mode(integer_list: &Vec<u32>) -> u32 {
    let mut occurrence_map = BTreeMap::new();

    let mut mode = 0;
    for num in integer_list {
        let count = occurrence_map.entry(num).or_insert(0);
        *count += 1;
        let count = *count;

        // if a mode exists yet
        if let Some(&mode_val) = occurrence_map.get(&mode) {
            // check if the current number has occurred more often than the mode
            if count > mode_val {
                mode = *num;    // update the mode
            }
        } else {
            mode = *num;    // just set the mode since there's no existing mode yet
        }
    }

    mode
}
