use std::collections::VecDeque;

pub enum CpuStatus { Output(i64), WaitForInput, Finished, Running }

#[derive(Clone)]
pub struct Cpu {
    code: Vec<i64>,
    pc: usize,
    input: VecDeque<i64>,
    relative_base: i64,
}

impl Cpu {
    pub fn new(code: &Vec<i64>) -> Cpu {
        return Cpu{
            code: code.to_vec(),
            pc: 0,
            input: VecDeque::new(),
            relative_base: 0,
        };
    }

    pub fn push_input(&mut self, input: i64) {
        self.input.push_back(input);
    }


    pub fn run_for_output(&mut self) -> i64 {
        loop {
            match self.step() {
                CpuStatus::Output(x) => return x,
                CpuStatus::Finished => panic!("Process should have an output"),
                _ => continue,
            }
        }
    }

    pub fn run(&mut self) -> () {
        loop {
            match self.step() {
                CpuStatus::Output(x) => println!("{}", x),
                CpuStatus::Finished => break,
                _ => continue,
            }
        };
    }

    pub fn step(&mut self) -> CpuStatus {
        let mut code: i64 = self.code[self.pc];
        let opcode = self.parse_code(&mut code, 100);
        let op1_mode = self.parse_code(&mut code, 10);
        let op2_mode = self.parse_code(&mut code, 10);
        let op3_mode = self.parse_code(&mut code, 10);

        match opcode {
            1 => { // add
                let op1 = self.get_operand_by_mode(op1_mode, 1);
                let op2 = self.get_operand_by_mode(op2_mode, 2);
                let op3: usize = self.get_address_by_mode(op3_mode, 3);
                self.ensure_memory(op3);
                self.code[op3] = op1 + op2;
                self.pc += 4;
            },
            2 => { // mul
                let op1 = self.get_operand_by_mode(op1_mode, 1);
                let op2 = self.get_operand_by_mode(op2_mode, 2);
                let op3: usize = self.get_address_by_mode(op3_mode, 3);
                self.ensure_memory(op3);
                self.code[op3] = op1 * op2;
                self.pc += 4;
            },
            3 => { // ld
                let param;
                match self.input.pop_front() {
                    Some(x) => param = x,
                    None => return CpuStatus::WaitForInput
                }
                let op1: usize = self.get_address_by_mode(op1_mode, 1) as usize;
                self.ensure_memory(op1);
                self.code[op1] = param;
                self.pc += 2;
            },
            4 => { // rd
                let op1 = self.get_operand_by_mode(op1_mode, 1);
                self.pc += 2;
                return CpuStatus::Output(op1 as i64);
            },
            5 => { //jt
                let op1 = self.get_operand_by_mode(op1_mode, 1);
                let op2 = self.get_operand_by_mode(op2_mode, 2);
                match op1 {
                    0 => self.pc += 3,
                    _ => self.pc = op2 as usize
                }
            },
            6 => { // jf
                let op1 = self.get_operand_by_mode(op1_mode, 1);
                let op2 = self.get_operand_by_mode(op2_mode, 2);
                match op1 {
                    0 => self.pc = op2 as usize,
                    _ => self.pc += 3,
                }
            },
            7 => { // lt
                let op1 = self.get_operand_by_mode(op1_mode, 1);
                let op2 = self.get_operand_by_mode(op2_mode, 2);
                let op3: usize = self.get_address_by_mode(op3_mode, 3);
                self.ensure_memory(op3);
                self.code[op3] = if op1 < op2 { 1 } else { 0 };
                self.pc += 4;
            },
            8 => { // eq
                let op1 = self.get_operand_by_mode(op1_mode, 1);
                let op2 = self.get_operand_by_mode(op2_mode, 2);
                let op3: usize = self.get_address_by_mode(op3_mode, 3);
                self.ensure_memory(op3);
                self.code[op3] = if op1 == op2 { 1 } else { 0 };
                self.pc += 4;
            },
            9 => {
                let op1 = self.get_operand_by_mode(op1_mode, 1);
                self.relative_base += op1;
                self.pc += 2;
            },
            99 => { // stop
                return CpuStatus::Finished
            }
            _ => panic!("Unknown opcode: {}", opcode),
        }
        CpuStatus::Running
    }

    fn parse_code(&mut self, code: &mut i64, digits: i64) -> i64 {
        let rem: i64 = *code % digits;
        *code = *code / digits;
        rem
    }

    fn get_operand_by_mode(&mut self, mode: i64, op_idx: usize) -> i64 {
        match mode {
            0 => {
                let idx = self.code[self.pc + op_idx] as usize;
                self.ensure_memory(idx);
                self.code[idx]
            },
            1 => {
                self.code[self.pc + op_idx]
            },
            2 => {
                let idx = (self.code[self.pc + op_idx] + self.relative_base) as usize;
                self.ensure_memory(idx);
                self.code[idx]
            },
            _ => panic!("Unknown operand mode: {}", mode),
        }
    }

    fn get_address_by_mode(&mut self, mode: i64, op_idx: usize) -> usize {
        let addr = match mode {
            0 => self.code[self.pc + op_idx],
            2 => self.code[self.pc + op_idx] + self.relative_base,
            _ => panic!("Invalid mode for writing: {}", mode),
        };
        if addr < 0 {
            panic!("Negative address encountered: {}", addr);
        }
        addr as usize
    }

    fn ensure_memory(&mut self, idx: usize) {
        if idx >= self.code.len() {
            self.code.resize(idx + 1, 0);
        }
    }
}

