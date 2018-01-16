use std::collections::HashSet;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

pub fn solve() -> (i32, i32){
    // Read in
    // for each line create hashset
    // check for existence
    // if exists, don't increment
    // if we get to the end, increment
    // return final count
    let mut count = 0;
    let mut second_count = 0;
    let input = get_input();

    for line in input.lines(){
        let mut valid = true;
        let mut more_valid = true;
        let mut row : HashSet<&str> = HashSet::new();
        let mut unique_word_val : HashSet<i32> = HashSet::new();

        let words = line.as_ref().unwrap().split_whitespace();

        for word in words {
            if row.contains(&word){
                debug!("{} is already in this phrase", word);
                break;
            } else {
                row.insert(&word);
                let word_val = numerical_val(&word);
                if unique_word_val.contains(&word_val){
                    more_valid = false;
                } else {
                    unique_word_val.insert(word_val);
                }
            }
        }

        if valid {
            count += 1;
            if more_valid {
                second_count += 1;
            }
        }
    }

    return (count, second_count);
}

fn get_input() -> BufReader<File> {
    let path = format!("{}/inputs/{}.txt", env!("CARGO_MANIFEST_DIR"), 4);
    let f = File::open(path).expect("Failed to open file");

    return BufReader::new(f);
}

fn numerical_val(input: &str) -> i32 {
    // map each letter in the string to a numerical value, from 1 - 26
    // return the sum of the map function
    return 0;
}