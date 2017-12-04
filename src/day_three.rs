pub fn solve(input: String) -> (i32,i32) {
    let coordinates = get_coordinates(input.parse::<i32>().unwrap());
    let first_answer = coordinates.0.abs() + coordinates.1.abs();
    return (first_answer, 0);
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