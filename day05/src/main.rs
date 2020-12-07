use std::io::{self, BufRead};

// let row_string: String = line.chars().take(7).map(|c| if c == 'F' { '0' } else { '1' }).collect();
// let row_number = isize::from_str_radix(&row_string, 2).unwrap();
//
// let col_string: String = line.chars().skip(7).take(3).map(|c| if c == 'L' { '0' } else { '1' }).collect();
// let col_number = isize::from_str_radix(&col_string, 2).unwrap();

// let seat_number = row_number * 8 + col_number;


fn main() {
    let stdin = io::stdin();
    let mut seat_numbers: Vec<isize> = Vec::new();
    for strange_line in stdin.lock().lines() {
        let convert_to_binary = |c| match c {
            'F' => '0',
            'B' => '1',
            'L' => '0',
            'R' => '1',
            _ => panic!("wrong char")
        };

        let line = strange_line.unwrap();
        let seat_string: String = line.chars().map(convert_to_binary).collect();
        let seat_number = isize::from_str_radix(&seat_string, 2).unwrap();
        seat_numbers.push(seat_number);
    }

    // Task 1
    seat_numbers.sort();
    println!("Highest seat is: {}", seat_numbers.last().unwrap());

    // Task 2
    for i in 1..seat_numbers.len() {
        if seat_numbers[i - 1] != seat_numbers[i] - 1 {
            println!("Missing: {}", seat_numbers[i] - 1);
        }
    }
}
