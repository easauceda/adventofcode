pub fn solve(input: String) -> (i32,i32) {
    let input_int = input.parse::<i32>().unwrap();

    let coordinates = get_coordinates(input_int);
    let first_answer = coordinates.0.abs() + coordinates.1.abs();

    let second_answer = get_next_value(input_int);


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
    // Use coordinates to determine neighbors
    // TODO: Solve how to make turns

    // if x = y: decrease y for turn
    // if x = -row_index && y = row_index: move down; decrease x only
    // if x = -row_index && y = -row_index: increase y
    // if x = row_index && y = -row_index: row done
    while row_index < 3 {

        while coordinates != (row_index, row_index) {
            coordinates = (coordinates.0, coordinates.1 + 1);

            debug!("{},{}", coordinates.0, coordinates.1);
        }

        while coordinates != (-row_index, row_index) {
            coordinates = (coordinates.0 - 1, coordinates.1);

            debug!("Moving left");
            debug!("{},{}", coordinates.0, coordinates.1);
        }

        while coordinates != (-row_index, -row_index) {
            coordinates = (coordinates.0, coordinates.1 - 1);

            debug!("Moving down");
            debug!("{},{}", coordinates.0, coordinates.1);
        }

        while coordinates != (row_index, -row_index) {
            coordinates = (coordinates.0 + 1, coordinates.1);

            debug!("Moving right");
            debug!("{},{}", coordinates.0, coordinates.1);
        }

        row_index += 1;
        coordinates = (coordinates.0 + 1, coordinates.1);
    }
    unimplemented!()
}

fn find_sum(coordinates: (i32, i32)) -> i32 {
    unimplemented!()
}