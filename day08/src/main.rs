use std::io::{self, BufRead};
use std::collections::HashSet;

#[derive(Debug, PartialEq)]
enum InstructionKind {
    NOP,
    ACC,
    JMP,
}

struct Instruction {
    kind: InstructionKind,
    arg: isize,
}

struct Program {
    instructions: Vec<Instruction>,
}

#[derive(Default)]
struct Context {
    accumulator: isize,
    program_counter: isize,
    visited_program_counters: HashSet<isize>,
}

impl Program {
    fn new() -> Program {
        Program { instructions: Vec::new() }
    }

    fn parse(&mut self) {
        let stdin = io::stdin();
        for strange_line in stdin.lock().lines() {
            let line = strange_line.unwrap();
            let mut iter = line.split_whitespace();

            let op_code: String = iter.next().unwrap().to_string();
            let arg: isize = iter.next().unwrap().parse::<isize>().unwrap();
            self.instructions.push(Program::create_instruction(&op_code, arg));
        }
    }

    fn create_instruction(op_code: &String, arg: isize) -> Instruction {
        match op_code.as_str() {
            "nop" => Instruction { kind: InstructionKind::NOP, arg: arg },
            "acc" => Instruction { kind: InstructionKind::ACC, arg: arg },
            "jmp" => Instruction { kind: InstructionKind::JMP, arg: arg },
            _ => panic!("unknown instruction: {}", op_code)
        }
    }

    fn dump(&self) {
        for (idx, instruction) in self.instructions.iter().enumerate() {
            println!("{}: {:?} {}", idx, instruction.kind, instruction.arg);
        }
    }

    fn debug_infinite_loop(&self, context: &mut Context) -> bool {
        loop {
            let instruction: &Instruction = &self.instructions[context.program_counter as usize];
            match instruction.kind {
                InstructionKind::NOP => {
                    context.program_counter += 1;
                }
                InstructionKind::ACC => {
                    context.accumulator += instruction.arg;
                    context.program_counter += 1;
                }
                InstructionKind::JMP => {
                    context.program_counter += instruction.arg;
                }
            }
            if context.visited_program_counters.contains(&context.program_counter) { return false; }
            context.visited_program_counters.insert(context.program_counter);

            if context.program_counter as usize >= self.instructions.len() { return true; }
        }
    }
}

impl Context {
    fn new() -> Context {
        Default::default()
    }

    fn dump(&self) {
        println!("acc={}, pc={}", self.accumulator, self.program_counter);
    }
}

fn task1() {
    let mut prog = Program::new();
    prog.parse();
    let mut context = Context::new();
    prog.debug_infinite_loop(&mut context);

    context.dump();
}


fn task2() {
    let mut prog = Program::new();
    prog.parse();

    for i in 0..prog.instructions.len() {
        let mut context = Context::new();

        {
            let instruction: &mut Instruction = &mut prog.instructions[i];
            match instruction.kind {
                InstructionKind::JMP => { instruction.kind = InstructionKind::NOP }
                InstructionKind::NOP => { instruction.kind = InstructionKind::JMP }
                _ => {}
            }
        }

        let terminated = prog.debug_infinite_loop(&mut context);
        if terminated {
            println!("found it @{}!", i);
            context.dump();
        }

        {
            let instruction: &mut Instruction = &mut prog.instructions[i];
            match instruction.kind {
                InstructionKind::JMP => { instruction.kind = InstructionKind::NOP }
                InstructionKind::NOP => { instruction.kind = InstructionKind::JMP }
                _ => {}
            }
        }
    }
}

fn main() {
    // task1();
    task2();
}
