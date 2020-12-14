use std::io::{self, BufRead};
use std::collections::HashMap;
use std::collections::BTreeMap;

enum Instruction {
    ASSIGN(u64, u64),
    MASK(String),
}

enum Instruction2 {
    ASSIGN(String, u64),
    MASK(String),
}

struct State {
    or_mask: u64,
    and_mask: u64,
    memory: HashMap<u64, u64>,
}

struct State2 {
    memory: BTreeMap<String, u64>,
    mask: String,
}

fn parse() -> Vec<Instruction> {
    let stdin = io::stdin();
    let mut result: Vec<Instruction> = Vec::new();
    for strange_line in stdin.lock().lines() {
        let line = strange_line.unwrap();
        let mut parts = line.split("=");
        let part1 = parts.next().unwrap();
        let part2 = parts.next().unwrap();

        if part1.starts_with("mask") {
            result.push(Instruction::MASK(part2.trim().to_string()));
        } else if part1.starts_with("mem") {
            let address: u64 = part1.trim_matches(|c: char| !c.is_numeric()).parse().unwrap();
            let value: u64 = part2.trim().parse().unwrap();
            result.push(Instruction::ASSIGN(address, value));
        } else {
            panic!("Unknown instruction: {}", part1);
        }
    }

    return result;
}

fn parse2() -> Vec<Instruction2> {
    let stdin = io::stdin();
    let mut result: Vec<Instruction2> = Vec::new();
    for strange_line in stdin.lock().lines() {
        let line = strange_line.unwrap();
        let mut parts = line.split("=");
        let part1 = parts.next().unwrap();
        let part2 = parts.next().unwrap();

        if part1.starts_with("mask") {
            result.push(Instruction2::MASK(part2.trim().to_string()));
        } else if part1.starts_with("mem") {
            let address_num: u64 = part1.trim_matches(|c: char| !c.is_numeric()).to_string().parse().unwrap();
            let address = format!("{:036b}", address_num);
            let value: u64 = part2.trim().parse().unwrap();
            result.push(Instruction2::ASSIGN(address, value));
        } else {
            panic!("Unknown instruction: {}", part1);
        }
    }

    return result;
}

fn print_input(input: &Vec<Instruction>) {
    for iter in input.iter() {
        match iter {
            Instruction::MASK(mask) => println!("mask = {}", mask),
            Instruction::ASSIGN(add, val) => println!("mem[{}] = {}", add, val),
        }
    }
}

fn run_task1(input: &Vec<Instruction>, state: &mut State) {
    for iter in input.iter() {
        match iter {
            Instruction::MASK(mask) => {
                let convert_or = |c| match c {
                    'X' => '0',
                    x => x,
                };
                let or_mask_str: String = mask.chars().map(convert_or).collect();
                state.or_mask = u64::from_str_radix(&or_mask_str, 2).unwrap();

                let convert_and = |c| match c {
                    'X' => '1',
                    x => x,
                };
                let and_mask_str: String = mask.chars().map(convert_and).collect();
                state.and_mask = u64::from_str_radix(&and_mask_str, 2).unwrap();
            }
            Instruction::ASSIGN(add, val) => {
                let masked_val = (*val | state.or_mask) & state.and_mask;
                state.memory.insert(*add, masked_val);
            }
        }
    }
}

#[allow(dead_code)]
fn task1() {
    let input = parse();
    let mut state = State { or_mask: 0, and_mask: !0, memory: HashMap::new() };
    run_task1(&input, &mut state);

    let mut sum = 0u64;
    for (k, v) in state.memory.iter() {
        println!("[{}] = {}", k, v);
        sum += v;
    }
    println!("sum: {}", sum);
}

fn apply_bitmaks(state: &State2, address: &String) -> String {
    let mut mask_iter = state.mask.chars();
    let mut result: Vec<char> = Vec::new();

    for address_iter in address.chars() {
        let mask_c: char = mask_iter.next().unwrap();
        let addr_c: char = address_iter;
        match (mask_c, addr_c) {
            ('0', x) => { result.push(x); }
            ('1', _) => { result.push('1'); }
            ('X', _) => { result.push('X'); }
            _ => panic!(""),
        }
    }

    return result.iter().collect();
}

fn is_address_disjunct(old: &String, new: &String) -> bool {
    let mut old_iter = old.chars();
    for new_iter in new.chars() {
        let old_c: char = old_iter.next().unwrap();
        let new_c: char = new_iter;
        match (old_c, new_c) {
            ('0', '1') => { return true; }
            ('1', '0') => { return true; }
            _ => { continue; }
        }
    }
    return false;
}

fn is_subset(old: &String, new: &String) -> bool {
    assert!(!is_address_disjunct(old, new));
    let mut old_iter = old.chars();
    for new_iter in new.chars() {
        let old_c: char = old_iter.next().unwrap();
        let new_c: char = new_iter;
        if old_c == 'X' && new_c != 'X' {
            return false;
        }
    }
    return true;
}

fn add_weak_address(old: &String, new: &String, val: u64, memory: &mut BTreeMap<String, u64>) {
    assert!(!is_subset(old, new));

    // println!("new: {}", new);
    // println!("old: {}", old);

    let mut old: Vec<char> = old.chars().collect();
    for (idx, new_iter) in new.chars().enumerate() {
        let old_c: char = old[idx];
        let new_c: char = new_iter;
        if old_c != 'X' {
            continue;
        } else {
            match new_c {
                'X' => { continue; }
                '1' => {
                    old[idx] = '0';
                    let result: String = old.iter().collect();
                    assert!(is_address_disjunct(&result, new));
                    memory.insert(result, val);
                    old[idx] = '1';
                }
                '0' => {
                    old[idx] = '1';
                    let result: String = old.iter().collect();
                    assert!(is_address_disjunct(&result, new));
                    memory.insert(result, val);
                    old[idx] = '0';
                }
                _ => panic!("Unknown char...."),
            }
        }
    }
}

fn print_memory(state: &State2) {
    for iter in &state.memory {
        println!("[{}] = {}", iter.0, iter.1);
    }
}

fn count_x(address: &String) -> u64 {
    2u64.pow(address.chars().filter(|x| *x == 'X').count() as u32)
}

fn run_task2(input: &Vec<Instruction2>, state: &mut State2) {
    for (_idx, iter) in input.iter().enumerate() {
        match iter {
            Instruction2::MASK(mask) => {
                state.mask = mask.clone();
                println!("set mask      : {}", mask);
            }
            Instruction2::ASSIGN(address, val) => {
                println!("assign address: {}", address);
                let new_address = apply_bitmaks(state, address);
                println!("new           : {}", new_address);

                let mut new_memory: BTreeMap<String, u64> = BTreeMap::new();
                new_memory.insert(new_address.clone(), *val);
                for old_address in &state.memory {
                    if is_address_disjunct(&old_address.0, &new_address) {
                        // The new address range is completely disjunct from the old one -> leaf old one as is
                        new_memory.insert(old_address.0.clone(), old_address.1.clone());
                    } else if is_subset(&old_address.0, &new_address) {
                        // The new address overwrites all the old addresses -> drop old address
                        continue;
                    } else {
                        // The addresses overlap -> keep non-overlapping range of old one
                        add_weak_address(&old_address.0, &new_address, old_address.1.clone(), &mut new_memory);
                    }
                }
                state.memory = new_memory;
            }
        }
    }
}

#[allow(dead_code)]
fn task2() {
    let input = parse2();
    let mut state = State2 { memory: BTreeMap::new(), mask: String::new() };
    run_task2(&input, &mut state);

    let mut sum = 0u64;
    let mut addresses = 0u64;
    for (k, v) in state.memory.iter() {
        println!("[{}] = {}", k, v);
        addresses += count_x(k);
        sum += v * count_x(k);
    }
    println!("sum: {}, addresses: {}", sum, addresses);
}

fn main() {
    // task1();
    task2();
}
