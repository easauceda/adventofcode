#[macro_use]
extern crate log;
extern crate env_logger;
extern crate clap;

mod util;
mod day_one;
mod day_two;

use clap::{App, Arg};

fn main() {
    env_logger::init().unwrap();
    
    let menu = App::new("Advent of Code 2017")
                    .version(env!("CARGO_PKG_VERSION"))
                    .author(env!("CARGO_PKG_AUTHORS"))
                    .arg(Arg::with_name("day"))
                    .get_matches();

    let day = menu.value_of("day").unwrap_or("");

    if day.is_empty() {
        println!("No day supplied.");
        std::process::exit(-1);
    }

    let result = match day {
        "1" => day_one::solve(util::get_input(day)),
        "2" => day_two::solve(util::get_input(day)),
        _ => {
            println!("Day has not been implemented.");
            std::process::exit(-1);
        }
    };
    
    println!("Day {} puzzle solutions:\n - Part 1: {}\n - Part 2: {}", day, result.0, result.1);
}
