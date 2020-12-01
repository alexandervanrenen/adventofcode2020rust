use std::io::{self, BufRead};
use std::time::Instant;

fn read_input() -> Vec<i32> {
    let mut v: Vec<i32> = Vec::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        v.push(line.unwrap().parse().unwrap());
    }
    return v;
}

fn do_task1() {
    let numbers = read_input();

    for a in numbers.iter() {
        for b in numbers.iter() {
            if a + b == 2020 {
                println!("Result: {}", a * b);
                return;
            }
        }
    }

    println!("No result found :(");
}

fn do_task2() {
    let numbers = read_input();

    for a in numbers.iter() {
        for b in numbers.iter() {
            if a + b > 2020 { continue; } // Optimized 8)
            for c in numbers.iter() {
                if a + b + c == 2020 {
                    println!("Result: {}", a * b * c);
                    return;
                }
            }
        }
    }

    println!("No result found :(");
}

fn main() {
    let start = Instant::now();
    // do_task1();
    do_task2();
    let elapsed = start.elapsed();
    println!("Time: {:?}", elapsed);
}
