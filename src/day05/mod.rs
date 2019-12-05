pub fn run (lines: Vec<String>) {
    let v: Vec<i32> = lines[0]
        .split(',')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();

    println!("part 1: {}", run_program(v.clone(), 1));
    println!("part 2: {}", run_program(v.clone(), 5));
}

fn run_program(mut v: Vec<i32>, input: i32) -> i32
{
    let mut diagnostic_code = 0;
    let mut ip = 0;
    loop {
        let mut opcode_parts = v[ip];
        let param_3_mode = opcode_parts / 10_000;
        opcode_parts -= param_3_mode * 10_000;
        let param_2_mode = opcode_parts / 1_000;
        opcode_parts -= param_2_mode * 1_000;
        let param_1_mode = opcode_parts / 100;
        opcode_parts -= param_1_mode * 100;
        let opcode = opcode_parts;

        match opcode {
            1 | 2 => {
                let a = get_param(&v, ip + 1, param_1_mode);
                let b = get_param(&v, ip + 2, param_2_mode);

                set_index(&mut v, ip + 3, if opcode == 1 { a + b } else { a * b });

                ip += 4;
            },
            3 => {
                set_index(&mut v, ip + 1, input);
                ip += 2;
            },
            4 => {
                let d = get_param(&v, ip + 1, param_1_mode);
                if diagnostic_code != 0 {
                    println!("previous output was an error: {}", diagnostic_code);
                }
                diagnostic_code = d;
                ip += 2;
            },
            5 | 6 => {
                let a = get_param(&v, ip + 1, param_1_mode);
                let is_non_zero = a != 0;
                let check_value = if opcode == 5 { true } else { false };
                if is_non_zero == check_value {
                    ip = get_param(&v, ip + 2, param_2_mode) as usize;
                } else {
                    ip += 3;
                }
            },
            7 => {
                let a = get_param(&v, ip + 1, param_1_mode);
                let b = get_param(&v, ip + 2, param_2_mode);

                set_index(&mut v, ip + 3, if a < b { 1 } else { 0 });

                ip += 4;
            },
            8 => {
                let a = get_param(&v, ip + 1, param_1_mode);
                let b = get_param(&v, ip + 2, param_2_mode);

                set_index(&mut v, ip + 3, if a == b { 1 } else { 0 });

                ip += 4;
            },
            99 => break,
            _ => panic!("unknown opcode {}", opcode)
        }
    }

    diagnostic_code
}

fn get_param(v: &Vec<i32>, index: usize, mode: i32) -> i32
{
    if mode == 0 { v[v[index] as usize] } else { v[index] }
}

fn set_index(v: &mut Vec<i32>, index: usize, value: i32)
{
    let target_index = v[index] as usize;
    v[target_index] = value;
}
