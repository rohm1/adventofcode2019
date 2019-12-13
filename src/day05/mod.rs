pub fn run (lines: Vec<String>) {
    let v: Vec<i64> = lines[0]
        .split(',')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();

    println!("part 1: {}", super::utils::intcode::Amp::new(v.clone()).run_program(1));
    println!("part 2: {}", super::utils::intcode::Amp::new(v.clone()).run_program(5));
}
