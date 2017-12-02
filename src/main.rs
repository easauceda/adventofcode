#[macro_use]
extern crate log;
extern crate env_logger;

mod day_one;
mod util;

fn main() {
    env_logger::init().unwrap();

    let input_one = util::get_input(1);
    let result_one = day_one::sum(input_one);
    println!("Day One Answer: {}", result_one);

}
