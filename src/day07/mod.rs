pub fn run (lines: Vec<String>) {
    let v: Vec<i32> = lines[0]
        .split(',')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();

    vec![1, 2]
        .iter()
        .for_each(|part| {
            println!("part {}: {}", *part, run_part(*part, v.clone()))
        });
}

fn run_part(part: i32, v: Vec<i32>) -> i32
{
    get_perms(if part == 1 { &[0, 1, 2, 3, 4] } else { &[5, 6, 7, 8, 9] })
        .iter()
        .map(|perm| {
            if part == 1 {
                let mut output = 0;
                perm
                    .iter()
                    .for_each(|p| {
                        output = Amp::new(v.clone()).run_program(Some(*p), output).unwrap_or(0);
                    });

                output
            } else {
                let mut amps = vec![Amp::new(v.clone()); 5];
                let mut signal = 0;
                let mut iter = perm.iter().cloned();

                for idx in (0..5).cycle() {
                    if let Some(s) = amps[idx].run_program(iter.next(), signal) {
                        signal = s;
                    } else {
                        break;
                    }
                }

                signal
            }
        })
        .max()
        .unwrap()
}

#[derive(Debug, Clone)]
struct Amp {
    mem: Vec<i32>,
    ip: usize,
}

impl Amp {
    fn new(mem: Vec<i32>) -> Amp {
        Amp {
            mem,
            ip: 0,
        }
    }

    fn run_program(&mut self, mut input: Option<i32>, signal: i32) -> Option<i32>
    {
        loop {
            let mut opcode_parts = self.mem[self.ip];
            let param_3_mode = opcode_parts / 10_000;
            opcode_parts -= param_3_mode * 10_000;
            let param_2_mode = opcode_parts / 1_000;
            opcode_parts -= param_2_mode * 1_000;
            let param_1_mode = opcode_parts / 100;
            opcode_parts -= param_1_mode * 100;
            let opcode = opcode_parts;

            match opcode {
                1 | 2 => {
                    let a = self.get_param(self.ip + 1, param_1_mode);
                    let b = self.get_param(self.ip + 2, param_2_mode);

                    self.set_index(self.ip + 3, if opcode == 1 { a + b } else { a * b });

                    self.ip += 4;
                },
                3 => {
                    if let Some(i) = input.take() {
                        self.set_index(self.ip + 1, i);
                    } else {
                        self.set_index(self.ip + 1, signal);
                    }
                    self.ip += 2;
                },
                4 => {
                    let diagnostic_code = self.get_param(self.ip + 1, param_1_mode);
                    self.ip += 2;
                    return Some(diagnostic_code);
                },
                5 | 6 => {
                    let a = self.get_param(self.ip + 1, param_1_mode);
                    let is_non_zero = a != 0;
                    let check_value = if opcode == 5 { true } else { false };
                    if is_non_zero == check_value {
                        self.ip = self.get_param(self.ip + 2, param_2_mode) as usize;
                    } else {
                        self.ip += 3;
                    }
                },
                7 => {
                    let a = self.get_param(self.ip + 1, param_1_mode);
                    let b = self.get_param(self.ip + 2, param_2_mode);

                    self.set_index(self.ip + 3, if a < b { 1 } else { 0 });

                    self.ip += 4;
                },
                8 => {
                    let a = self.get_param(self.ip + 1, param_1_mode);
                    let b = self.get_param(self.ip + 2, param_2_mode);

                    self.set_index(self.ip + 3, if a == b { 1 } else { 0 });

                    self.ip += 4;
                },
                99 => break,
                _ => panic!("unknown opcode {}", opcode)
            }
        }

        None
    }

    fn get_param(&mut self, index: usize, mode: i32) -> i32
    {
        if mode == 0 { self.mem[self.mem[index] as usize] } else { self.mem[index] }
    }

    fn set_index(&mut self, index: usize, value: i32)
    {
        let target_index = self.mem[index] as usize;
        self.mem[target_index] = value;
    }
}

// Get full list of permutations of given slice
fn get_perms(vals: &[i32]) -> Vec<Vec<i32>>
{
    if vals.len() == 1 {
        return vec![vals.to_vec()];
    }

    let mut perms = vec![];

    for idx in 0..vals.len() {
        let mut to_ch = vals.to_vec();
        let own = to_ch.remove(idx);

        let children = get_perms(&to_ch);
        for mut c in children {
            c.push(own);
            perms.push(c);
        }
    }

    perms
}
