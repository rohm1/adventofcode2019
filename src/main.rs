use std::env::args;
use std::process::exit;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
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
        test = args[2].eq(&("1".to_string()));
    }

    let file_name= format!("data/day{:#02}/input{}.txt", day, if test { "_test" } else { "" });
    let lines = utils::file_to_lines(&file_name.to_string());

    match day {
        1 => day01::run(lines),
        2 => day02::run(lines),
        3 => day03::run(lines),
        4 => day04::run(lines),
        5 => day05::run(lines),
        6 => day06::run(lines),
        7 => day07::run(lines),
        8 => day08::run(lines),
        9 => day09::run(lines),
        _ => {
            println!("unknown day {}", day);
            exit(1);
        }
    }
    exit(0);
}
