use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut valid_pw_count = 0;

    for strange_line in stdin.lock().lines() {
        let line = strange_line.unwrap();
        let mut iter = line.chars().peekable();

        let min: usize = parse_number(&mut iter);
        assert_eq!('-', parse_char(&mut iter));
        let max: usize = parse_number(&mut iter);
        assert_eq!(' ', parse_char(&mut iter));
        let c: char = parse_char(&mut iter);
        assert_eq!(':', parse_char(&mut iter));
        assert_eq!(' ', parse_char(&mut iter));
        let pw = parse_string(&mut iter);

        println!("line: min: '{}', max: '{}', c: '{}', pw: '{}'", min, max, c, pw);

        valid_pw_count += if is_valid_password(min, max, c, &pw) { 1 } else { 0 };
    }

    println!("valid_pw_count: {}", valid_pw_count);
}

fn is_digit(c: char) -> bool {
    return '0' <= c && c <= '9';
}

fn parse_number(iter: &mut std::iter::Peekable<std::str::Chars>) -> usize {
    let mut buffer = String::new();

    while is_digit(*iter.peek().unwrap()) {
        buffer.push(*iter.peek().unwrap());
        iter.next();
    }

    return buffer.parse::<usize>().unwrap();
}

fn parse_char(iter: &mut std::iter::Peekable<std::str::Chars>) -> char {
    let res = *iter.peek().unwrap();
    iter.next();
    return res;
}

fn parse_string(iter: &mut std::iter::Peekable<std::str::Chars>) -> String {
    iter.collect::<String>() // BÄHM, Rust power !!
}

fn is_valid_password(min: usize, max: usize, c: char, pw: &String) -> bool {
    // Task 1:
    // let occurrences = pw.chars().filter(|ele| *ele == c).count();
    // return min <= occurrences && occurrences <= max;

    // Task 2:
    let hits = pw.chars().enumerate().filter(|&(idx, ele)| ele == c && (idx + 1 == min || idx + 1 == max)).count();
    return hits == 1;
}

