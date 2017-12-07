use std::collections::HashMap;

pub fn solve(input: String) -> (i32,i32) {
    let input_int = input.parse::<i32>().unwrap();

    let coordinates = get_coordinates(input_int);
    let first_answer = coordinates.0.abs() + coordinates.1.abs();

    let second_answer = get_next_sum_fixed(input_int);


    return (first_answer, second_answer);
}

fn get_coordinates(target: i32) -> (i32, i32){
    let mut row_index = 0;
    let mut rightmost_num = 1;

    let x : i32;
    let y : i32;

    while rightmost_num < target {
        row_index += 1;
        rightmost_num += 8 * row_index;
        debug!("Current Rightmost Number: {}", rightmost_num);
    }

    debug!("Current Row: {}", row_index);

    let decrement = 2 * row_index;

    let bottom_left = rightmost_num - decrement;
    let top_left = bottom_left - decrement;
    let top_right = top_left - decrement;

    if target <= rightmost_num && target > bottom_left {
        y = -row_index;
        x = -(rightmost_num - row_index - target);
    } else if target <= bottom_left && target > top_left {
        x = -row_index;
        y = bottom_left - row_index - target;
    } else if target <= top_left && target > top_right {
        y = row_index;
        x = top_left - row_index - target;
    } else {
        x = row_index;
        y = -(top_right - row_index - target);
    }

    debug!("{}, {}", x, y);
    return (x, y);
}

fn get_next_value(input: i32) -> i32 {
    let mut row_index = 1;
    let mut coordinates = (1,0);
    let mut spiral_struct : HashMap<(i32, i32), i32> = HashMap::new();

    //insert initial value
    spiral_struct.insert((0,0), 1);

    loop {
        let mut val : i32 = 0;

        if val > input {
            return val;
        }

        while coordinates != (row_index, row_index) {
            val = find_sum(&coordinates, &mut spiral_struct);
            spiral_struct.insert(coordinates, val);

            if val > input {
                return val;
            }

            debug!("Inserting {} at {},{}", val, coordinates.0, coordinates.1);
            debug!("{},{}", coordinates.0, coordinates.1);
            coordinates = (coordinates.0, coordinates.1 + 1);
        }

        while coordinates != (-row_index, row_index) {
            val = find_sum(&coordinates, &mut spiral_struct);
            spiral_struct.insert(coordinates, val);

            if val > input {
                return val;
            }

            debug!("Inserting {} at {},{}", val, coordinates.0, coordinates.1);
            debug!("Moving left");
            debug!("{},{}", coordinates.0, coordinates.1);
            coordinates = (coordinates.0 - 1, coordinates.1);
        }

        while coordinates != (-row_index, -row_index) {
            val = find_sum(&coordinates, &mut spiral_struct);
            spiral_struct.insert(coordinates, val);

            if val > input {
                return val;
            }

            debug!("Inserting {} at {},{}", val, coordinates.0, coordinates.1);
            debug!("Moving down");
            debug!("{},{}", coordinates.0, coordinates.1);
            coordinates = (coordinates.0, coordinates.1 - 1);
        }

        while coordinates != (row_index, -row_index) {
            val = find_sum(&coordinates, &mut spiral_struct);
            spiral_struct.insert(coordinates, val);

            if val > input {
                return val;
            }
            
            debug!("Inserting {} at {},{}", val, coordinates.0, coordinates.1);
            debug!("Moving right");
            coordinates = (coordinates.0 + 1, coordinates.1);
            debug!("{},{}", coordinates.0, coordinates.1);
        }

        val = find_sum(&coordinates, &mut spiral_struct);
        spiral_struct.insert(coordinates, val);
        row_index += 1;
        coordinates = (coordinates.0 + 1, coordinates.1);
    }
}

fn get_next_sum_fixed(input : i32) -> i32 {
    let mut row_index = 1;
    let mut coordinates = (1,0);
    let mut spiral_struct : HashMap<(i32, i32), i32> = HashMap::new();

    //insert initial value
    spiral_struct.insert((0,0), 1);

    let mut direction = "up";

    let mut val = 0;

    while val < input {

        debug!("Evaluating {},{}", coordinates.0, coordinates.1);
        val = find_sum(&coordinates, &mut spiral_struct);
        spiral_struct.insert(coordinates, val);

        debug!("Inserting {} at {},{}", val, coordinates.0, coordinates.1);

        if coordinates == (row_index, -row_index) {

            coordinates = (coordinates.0 + 1, coordinates.1);
            
            val = find_sum(&coordinates, &mut spiral_struct);
            spiral_struct.insert(coordinates, val);

            direction = "up";
            row_index += 1;

            debug!("New Row! Inserting {} at {},{}", val, coordinates.0, coordinates.1);
        }


        match direction {
            "up" => {
                coordinates = (coordinates.0, coordinates.1 + 1);
            },
            "down" => {
                coordinates = (coordinates.0, coordinates.1 - 1);
            },
            "left" => {
                coordinates = (coordinates.0 - 1, coordinates.1);
            },
            "right" => {
                coordinates = (coordinates.0 + 1, coordinates.1);
            },
            _ => {
                println!("Unknown Direction: {}", direction);
            }
        }

        if coordinates == (row_index, row_index) {
            direction = "left";
        } else if coordinates == (-row_index, row_index) {
            direction = "down";
        } else if coordinates == (-row_index, -row_index) {
            direction = "right";
        }
    }

    return val;
}


fn find_sum(coordinates: &(i32, i32), spiral_struct: &mut HashMap<(i32, i32), i32>) -> i32 {
    let x = coordinates.0;
    let y = coordinates.1;
    
    let mut sum = 0;

    let neighbors = vec![(x + 1, y + 1), (x, y + 1), (x - 1, y + 1), (x - 1, y), (x - 1, y - 1), (x, y - 1), (x + 1, y - 1), (x + 1, y)];

    for coordinate in neighbors {
        if spiral_struct.contains_key(&coordinate) {
            debug!("Adding neighbor {},{}", coordinate.0, coordinate.1);
            sum += *spiral_struct.get(&coordinate).unwrap();
        }
    }
    return sum;
}