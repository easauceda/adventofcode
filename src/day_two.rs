pub fn solve(input: String) -> (u32, u32){
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

fn parse(input: String) -> Vec<Vec<u32>> {
    let mut rows = Vec::new();
    let input_lines = input.split("\n");

    for line in input_lines {
        let mut row : Vec<u32> = Vec::new();

        for val in line.split_whitespace(){
            row.push(val.parse::<u32>().unwrap());
        }

        rows.push(row);
    }
    
    return rows;
}

fn diff(row: &Vec<u32>) -> u32 {
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

fn even(row: &Vec<u32>) -> u32 {
    unimplemented!();
}