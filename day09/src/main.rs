use std::io::{self, BufRead};
use std::collections::HashMap;
use std::collections::VecDeque;

struct XMAS {
    window_size: usize,
    numbers: VecDeque<usize>,
    possible_sums: HashMap<usize, usize>,
}

impl XMAS {
    fn push(&mut self, new_num: usize) {
        if self.window_size == self.numbers.len() {
            self.check_valid(new_num);
            self.remove_old_number();
        }

        self.add_new_number(new_num);
    }

    fn dump(&self) {
        for (k, v) in self.possible_sums.iter() {
            println!("{} -> {}", k, v);
        }
    }

    fn check_valid(&self, new_num: usize) {
        if !self.possible_sums.contains_key(&new_num) {
            println!("Not a possible sum: {}", new_num);
        }
    }

    fn remove_old_number(&mut self) {
        let old_num = *self.numbers.front().unwrap();
        self.numbers.pop_front();
        for num in self.numbers.iter() {
            if *num == old_num { continue; } // Only different values
            let old_sum = num + old_num;
            *self.possible_sums.get_mut(&old_sum).unwrap() -= 1;
            if *self.possible_sums.get(&old_sum).unwrap() == 0usize {
                self.possible_sums.remove(&old_sum);
            }
        }
    }

    fn add_new_number(&mut self, new_num: usize) {
        for num in self.numbers.iter() {
            if *num == new_num { continue; } // Only different values
            let new_sum = num + new_num;
            let counter = self.possible_sums.entry(new_sum).or_insert(0);
            *counter += 1;
        }
        self.numbers.push_back(new_num);
    }
}

fn task1() {
    let mut xmas = XMAS { window_size: 25, numbers: VecDeque::new(), possible_sums: HashMap::new() };
    let stdin = io::stdin();
    for strange_line in stdin.lock().lines() {
        let number: usize = strange_line.unwrap().parse().unwrap();
        xmas.push(number);
    }
}

fn task2() {
    let mut numbers = Vec::new();
    let stdin = io::stdin();
    for strange_line in stdin.lock().lines() {
        let number: usize = strange_line.unwrap().parse().unwrap();
        numbers.push(number);
    }

    let goal = 36845998usize;

    for start in 0..numbers.len() {
        let mut sum = 0usize;
        let mut smallest_num = numbers[start];
        let mut largest_num = numbers[start];
        for i in start..numbers.len() {
            let num: usize = numbers[i];
            sum += num;
            smallest_num = smallest_num.min(num);
            largest_num = largest_num.max(num);
            if sum == goal { println!("Found range [{}, {}] -> {}", start, i, smallest_num + largest_num); }
            if sum > goal { break; }
        }
    }
}

fn main() {
    // task1();
    task2();
}
