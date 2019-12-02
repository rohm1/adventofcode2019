use std::env::args;
use std::process::exit;

mod day01;
mod day02;
mod utils;

fn main() {
    let args: Vec<String> = args().collect();
    let mut test = false;

    if args.len() != 2 && args.len() != 3 {
        println!("Usage: cargo {} XX", args[0]);
        exit(1);
    }

    let day : u32 = match args[1].trim().parse() {
        Ok(_d) => _d,
        Err(_e) => {
            println!("Error: {}", _e);
            exit(1)
        }
    };

    if args.len() == 3 {
        test = args[2].eq("1");
    }

    let file_name= format!("data/day{:#02}/input{}.txt", day, if test { "_test" } else { "" });
    let lines = utils::file_to_lines(&file_name.to_string());

    match day {
        1 => day01::run(lines),
        2 => day02::run(lines),
        _ => {
            println!("unknown day {}", day);
            exit(1);
        }
    }
    exit(0);
}