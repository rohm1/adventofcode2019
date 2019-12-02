pub fn run (lines: Vec<String>) {
    let v: Vec<i32> = lines[0]
        .split(',')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();

    println!("part 1: {}", run_program(v.clone(), 12, 2));

    let mut f = false;
    for noun in 0..=99 {
        for verb in 0..=99 {
            if run_program(v.clone(), noun, verb) == 19690720 {
                println!("part 2: {}", 100 * noun + verb);
                f = true;
                break;
            }
        }

        if f {
            break;
        }
    }
}

fn run_program(mut v: Vec<i32>, noun: i32, verb: i32) -> i32 {
    v[1] = noun;
    v[2] = verb;

    let mut pos = 0;
    loop {
        let opcode = v[pos];
        match opcode {
            1 | 2 => {
                let a = v[v[pos + 1] as usize];
                let b = v[v[pos + 2] as usize];

                let target_index = v[pos + 3] as usize;
                v[target_index] = if opcode == 1 { a + b } else { a * b };

                pos += 4;
            }
            99 => break,
            _ => panic!("unknown opcode {}", opcode)
        }
    }

    v[0]
}
