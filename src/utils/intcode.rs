#[derive(Debug, Clone)]
pub struct Amp {
    mem: Vec<i64>,
    ip: usize,
    rb: i64,
}

impl Amp {
    pub fn new(mem: Vec<i64>) -> Amp {
        Amp {
            mem,
            ip: 0,
            rb: 0,
        }
    }

    pub fn run_program(&mut self, input: i64) -> i64
    {
        let mut diagnostic_code = 0;

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

                    self.set_index(self.ip + 3, if opcode == 1 { a + b } else { a * b }, param_3_mode);

                    self.ip += 4;
                },
                3 => {
                    self.set_index(self.ip + 1, input, param_1_mode);
                    self.ip += 2;
                },
                4 => {
                    let d = self.get_param(self.ip + 1, param_1_mode);
                    if diagnostic_code != 0 {
                        println!("previous output was an error: {}", diagnostic_code);
                    }
                    diagnostic_code = d;
                    self.ip += 2;
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

                    self.set_index(self.ip + 3, if a < b { 1 } else { 0 }, param_3_mode);

                    self.ip += 4;
                },
                8 => {
                    let a = self.get_param(self.ip + 1, param_1_mode);
                    let b = self.get_param(self.ip + 2, param_2_mode);

                    self.set_index(self.ip + 3, if a == b { 1 } else { 0 }, param_3_mode);

                    self.ip += 4;
                },
                9 => {
                    self.rb += self.get_param(self.ip + 1, param_1_mode);
                    self.ip += 2;
                },
                99 => break,
                _ => panic!("unknown opcode {}", opcode)
            }
        }

        diagnostic_code
    }

    fn get_param(&mut self, index: usize, mode: i64) -> i64
    {
        let index = self.get_index(index, mode);

        if index >= self.mem.len() {
            return 0;
        }

        self.mem[index]
    }

    fn set_index(&mut self, index: usize, value: i64, mode: i64)
    {
        let index = self.get_index(index, mode);

        if index >= self.mem.len() {
            for _i in self.mem.len()..3*index {
                self.mem.push(0);
            }
        }

        self.mem[index] = value;
    }

    fn get_index(&mut self, index: usize, mode: i64) -> usize
    {
        match mode {
            0 | 2 => {
                let rb = if mode == 2 { self.rb } else { 0 };
                (if index >= self.mem.len() { 0 } else { self.mem[index] } + rb) as usize
            },
            1 => index,
            _ => panic!("unknown mode {}", mode)
        }
    }
}
