use super::utils;

pub fn run (lines: Vec<String>) {
    let lines = utils::vec_to_int(lines);

    let mut sum = 0;

    for mass in lines.clone() {
        sum += fuel_for_mass(mass);
    }

    println!("part 1: {}", sum);

    sum = 0;
    for module in lines.clone() {
        sum += fuel_for_module(module);
    }

    println!("part 2: {}", sum);
}

fn fuel_for_module(mass: i32) -> i32 {
    let mut mass = mass;
    let mut fuel = 0;
    loop {
        let r = fuel_for_mass(mass);
        if r <= 0 {
            break;
        }
        mass = r;
        fuel += r;
    }
    fuel
}

fn fuel_for_mass(mass: i32) -> i32 {
    let f: i32 = mass / 3;
    f - 2
}
