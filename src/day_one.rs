pub fn solve(input: String) -> (u32, u32){
    let mut sum = 0;
    // Convert the input string into a vector of chars
    let chars : Vec<_> = input.chars().collect();
    let step = chars.len() / 2;
    
    for i in 0..chars.len(){

        let current = chars[i];
        let next : char;

        let mut next_char_pos = i + step;

        debug!("Next Position: {}", next_char_pos);
        
        if next_char_pos > chars.len() - 1 {
            // step should loop around to the correct index.
            next_char_pos -= chars.len();
            next = chars[next_char_pos];
        } else {
            next = chars[next_char_pos];
        }
        
        if current == next {
            debug!("Adding to total");
            sum += current.to_digit(10).unwrap();
        } else {
            debug!("No Addition");
        }
        debug!("===========");
    }
    return (sum, sum);
}
