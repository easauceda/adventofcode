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
    let input = get_input();

    for line in input.lines(){
        let mut valid = true;
        let mut row : HashSet<&str> = HashSet::new();



        let words = line.as_ref().unwrap().split_whitespace();

        for word in words {
            if row.contains(&word){
                debug!("{} is already in this phrase", word);
                break;
            } else {
                row.insert(&word);
            }
        }

        if valid {
            count += 1;
        }
    }
    return (count,0);
}

fn get_input() -> BufReader<File> {
    let path = format!("{}/inputs/{}.txt", env!("CARGO_MANIFEST_DIR"), 4);
    let f = File::open(path).expect("Failed to open file");

    return BufReader::new(f);
}