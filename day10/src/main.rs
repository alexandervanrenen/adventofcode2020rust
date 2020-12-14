use std::io::{self, BufRead};

#[allow(dead_code)]
fn task1() {
    let stdin = io::stdin();
    let mut numbers: Vec<usize> = Vec::new();
    for strange_line in stdin.lock().lines() {
        let number: usize = strange_line.unwrap().parse().unwrap();
        numbers.push(number);
    }

    numbers.sort();

    let mut cnt1: usize = 0;
    let mut cnt2: usize = 0;
    let mut cnt3: usize = 1; // Last step always requires a 3 jump

    let mut current_joltage: usize = 0;

    for num in numbers.iter() {
        match num - current_joltage {
            1 => cnt1 += 1,
            2 => cnt2 += 1,
            3 => cnt3 += 1,
            _ => panic!("no adapter for {}", num),
        }
        current_joltage = *num;
    }

    println!("cnt1: {}", cnt1);
    println!("cnt2: {}", cnt2);
    println!("cnt3: {}", cnt3);
    println!("res: {}", cnt1 * cnt3);
}

fn task2() {
    let stdin = io::stdin();
    let mut numbers: Vec<usize> = Vec::new();
    numbers.push(0);
    for strange_line in stdin.lock().lines() {
        let number: usize = strange_line.unwrap().parse().unwrap();
        numbers.push(number);
    }
    numbers.sort();
    numbers.push(*numbers.last().unwrap() + 3);

    let mut fixed: Vec<bool> = Vec::new();
    fixed.push(true);
    for i in 1..(numbers.len() - 1) {
        fixed.push(numbers[i + 1] - numbers[i - 1] > 3);
    }
    fixed.push(true);

    let mut max_mutable = 0;
    let mut cur_mutable = 0;
    let mut last_fixed_number = 0;
    let mut options: usize = 1;
    for (idx, num) in numbers.iter().enumerate() {
        println!("{} {}", num, fixed[idx]);
        if fixed[idx] {
            match cur_mutable {
                0 => {}
                1 => { options *= 2; }
                2 => {
                    options *= match num - last_fixed_number {
                        3 => { 4 }
                        4 => { 3 }
                        5 => { 3 }
                        _ => { panic!("diff too large"); }
                    };
                }
                3 => {
                    options *= match num - last_fixed_number {
                        4 => { 7 }
                        5 => { 6 }
                        6 => { 5 }
                        _ => { panic!("diff too large") }
                    };
                }
                _ => panic!("does not happen in the example")
            }

            max_mutable = max_mutable.max(cur_mutable);
            cur_mutable = 0;
            last_fixed_number = *num;
        } else {
            cur_mutable += 1;
        }
    }

    println!("Options: {}", options);
}

fn main() {
    task2();
}
