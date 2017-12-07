pub fn solve(input: String) -> (i32, i32){
    debug!("Input is: {}", input);

    let mut sum = 0;
    let mut even_sum = 0;

    // Fill a 2D vector with values.
    let rows = parse(input);
    
    // Find the largest and smallest values for each row.
    for row in rows {
        debug!("Row Length: {}", row.len());
        sum += diff(&row);
        even_sum += even(&row);
    }
    
    return (sum, even_sum);
}

fn parse(input: String) -> Vec<Vec<i32>> {
    let mut rows = Vec::new();
    let input_lines = input.split("\n");

    for line in input_lines {
        let mut row : Vec<i32> = Vec::new();

        for val in line.split_whitespace(){
            row.push(val.parse::<i32>().unwrap());
        }

        rows.push(row);
    }
    
    return rows;
}

fn diff(row: &Vec<i32>) -> i32 {
    let mut smallest = &row[0];
    let mut largest = &row[0];

    for val in row {

        if val > largest {
            largest = val;
        }

        if val < smallest {
            smallest = val;
        }
    }
    return largest - smallest;
}

fn even(row: &Vec<i32>) -> i32 {
    // For each number, search the row for a number than evenly divides into it
    //if found, break and return the result
    for x in 0..row.len() {
        for y in (x+1)..row.len(){
            let a = &row[x];
            let b = &row[y];

            debug!("Value for A: {}", a);
            debug!("Value for B: {}", b);

            if a % b == 0 {
                debug!("Returning: {}", (a/b));
                return a / b;
            } else if b % a == 0 {
                debug!("Returning: {}", (b/a));
                return b / a;
            }
        }
    }
    debug!("Should not reach this line");
    return 1;
}