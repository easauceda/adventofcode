#[macro_use]
extern crate log;
extern crate env_logger;


mod util;

fn main() {
    env_logger::init().unwrap();

    let input_one = util::get_input(1);


}
