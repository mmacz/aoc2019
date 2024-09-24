use std::collections::VecDeque;

pub enum CpuStatus { Output(i32), WaitForInput, Finished, Running }

#[derive(Clone)]
pub struct Cpu {
    code: Vec<i32>,
    pc: usize,
    input: VecDeque<i32>,
}

impl Cpu {
    pub fn new(code: &Vec<i32>) -> Cpu {
        return Cpu{
            code: code.to_vec(),
            pc: 0,
            input: VecDeque::new(),
        };
    }

    pub fn push_input(&mut self, input: i32) {
        self.input.push_back(input);
    }

    pub fn process_for_output(&mut self) -> i32 {
        match self.process() {
            CpuStatus::Output(x) => x,
            _ => panic!("Process should have an output")
        }
    }

    pub fn process(&mut self) -> CpuStatus {
        loop {
            let mut code: i32 = self.code[self.pc];
            let opcode: i32 = self.parse_code(&mut code, 100);
            let op1_mode: i32 = self.parse_code(&mut code, 10);
            let op2_mode: i32 = self.parse_code(&mut code, 10);
            let _op3_mode: i32 = self.parse_code(&mut code, 10);

            match opcode {
                1 => { // add
                    let op1: i32 = self.get_operand_by_mode(op1_mode, 1);
                    let op2: i32 = self.get_operand_by_mode(op2_mode, 2);
                    let op3: i32 = self.get_operand_by_mode(1, 3);
                    self.code[op3 as usize] = op1 + op2;
                    self.pc += 4;
                },
                2 => { // mul
                    let op1: i32 = self.get_operand_by_mode(op1_mode, 1);
                    let op2: i32 = self.get_operand_by_mode(op2_mode, 2);
                    let op3: i32 = self.get_operand_by_mode(1, 3);
                    self.code[op3 as usize] = op1 * op2;
                    self.pc += 4;
                },
                3 => { // ld
                    let param: i32;
                    match self.input.pop_front() {
                        Some(x) => param = x,
                        None => return CpuStatus::WaitForInput
                    }
                    let op1: usize = self.get_operand_by_mode(1, 1) as usize;
                    self.code[op1] = param;
                    self.pc += 2;
                },
                4 => { // rd
                    let op1: usize = self.get_operand_by_mode(1, 1) as usize;
                    self.pc += 2;
                    return CpuStatus::Output(self.code[op1]);
                },
                5 => { //jt
                    let op1: i32 = self.get_operand_by_mode(op1_mode, 1);
                    let op2: i32 = self.get_operand_by_mode(op2_mode, 2);
                    match op1 {
                        0 => self.pc += 3,
                        _ => self.pc = op2 as usize
                    }
                },
                6 => { // jf
                    let op1: i32 = self.get_operand_by_mode(op1_mode, 1);
                    let op2: i32 = self.get_operand_by_mode(op2_mode, 2);
                    match op1 {
                        0 => self.pc = op2 as usize,
                        _ => self.pc += 3,
                    }
                },
                7 => { // lt
                    let op1: i32 = self.get_operand_by_mode(op1_mode, 1);
                    let op2: i32 = self.get_operand_by_mode(op2_mode, 2);
                    let op3: i32 = self.get_operand_by_mode(1, 3);
                    self.code[op3 as usize] = if op1 < op2 { 1 } else { 0 };
                    self.pc += 4;
                },
                8 => { // eq
                    let op1: i32 = self.get_operand_by_mode(op1_mode, 1);
                    let op2: i32 = self.get_operand_by_mode(op2_mode, 2);
                    let op3: i32 = self.get_operand_by_mode(1, 3);
                    self.code[op3 as usize] = if op1 == op2 { 1 } else { 0 };
                    self.pc += 4;
                },
                99 => { // stop
                    return CpuStatus::Finished
                }
                _ => panic!("Unknown opcode: {}", opcode),
            }
        }
    }

    fn parse_code(&mut self, code: &mut i32, digits: i32) -> i32 {
        let rem: i32 = *code % digits;
        *code = *code / digits;
        rem
    }

    fn get_operand_by_mode(&mut self, mode: i32, op_idx: usize) -> i32 {
        let mut op: i32;
        match mode {
            0 => {
                op = self.code[self.pc + op_idx];
                op = self.code[op as usize];
            },
            1 => {
                op = self.code[self.pc + op_idx];
            }
            _ => panic!("Unknown operand mode: {}", mode),
        }
        op
    }
}
