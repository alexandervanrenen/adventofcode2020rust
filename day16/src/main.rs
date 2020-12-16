use std::io::{self, BufRead};
use regex::Regex;
use std::collections::HashSet;

struct Range(usize, usize);

struct Rule {
    name: String,
    ranges: Vec<Range>,
}

#[derive(Debug, Clone)]
struct Ticket {
    numbers: Vec<usize>,
}

struct Input {
    rules: Vec<Rule>,
    my_ticket: Ticket,
    tickets: Vec<Ticket>,
}

impl Rule {
    fn new(line: &str) -> Rule {
        let re = Regex::new(r"^([a-z ]+): ([0-9]+)-([0-9]+) or ([0-9]+)-([0-9]+)").unwrap();
        let caps = re.captures(line).unwrap();

        let name = caps.get(1).map_or("", |m| m.as_str()).to_string();

        // Parse all rules
        let mut idx = 2;
        let mut ranges: Vec<Range> = Vec::new();
        while caps.get(idx).is_some() {
            let from = caps.get(idx + 0).map_or("", |m| m.as_str());
            let to = caps.get(idx + 1).map_or("", |m| m.as_str());
            ranges.push(Range(from.parse().unwrap(), to.parse().unwrap()));
            idx += 2;
        }

        return Rule { name, ranges };
    }

    fn dump(&self) {
        print!("{}: ", self.name);
        for range in &self.ranges {
            print!("{}-{}, ", range.0, range.1);
        }
        println!();
    }

    fn accepts(&self, num: usize) -> bool {
        self.ranges.iter().any(|range| range.0 <= num && num <= range.1)
    }
}

impl Ticket {
    fn new(line: &str) -> Ticket {
        Ticket { numbers: line.split(",").map(|s| s.trim().parse().unwrap()).collect() }
    }

    fn dump(&self) {
        for number in &self.numbers {
            print!("{},", number);
        }
        println!();
    }
}

impl Input {
    fn new() -> Input {
        let stdin = io::stdin();
        let mut line_iter = stdin.lock().lines();
        let mut line: String = line_iter.next().unwrap().unwrap();

        // Parse rules
        let mut rules: Vec<Rule> = Vec::new();
        while line != "" {
            rules.push(Rule::new(&line));
            line = line_iter.next().unwrap().unwrap();
        }

        // Parse my ticket
        line_iter.next().unwrap().unwrap(); // Skip header 'your ticket:'
        line = line_iter.next().unwrap().unwrap();
        let my_ticket = Ticket::new(&line);

        // Parse other tickets
        line_iter.next().unwrap().unwrap(); // Skip empty line
        line_iter.next().unwrap().unwrap(); // Skip header 'nearby tickets:'
        let mut tickets: Vec<Ticket> = Vec::new();
        while let Some(line) = line_iter.next() {
            tickets.push(Ticket::new(&line.unwrap()));
        }

        return Input { rules, my_ticket, tickets };
    }

    fn dump(&self) {
        println!("------------------------------------------");
        println!("Rules ...");
        for rule in &self.rules {
            rule.dump();
        }

        println!("\nMy ticket ...");
        self.my_ticket.dump();

        println!("\nOther tickets ...");
        for ticket in &self.tickets {
            ticket.dump();
        }
        println!("------------------------------------------");
    }

    fn is_value_valid_in_any_rule(&self, num: usize) -> bool {
        self.rules.iter().any(|rule| rule.accepts(num))
    }

    fn task1(&self) -> usize {
        let mut sum = 0;
        for ticket in self.tickets.iter() {
            sum += ticket.numbers.iter().filter(|x| !self.is_value_valid_in_any_rule(**x)).fold(0, |acc, x| acc + x);
        }
        return sum;
    }

    fn get_all_valid_tickets(&self) -> Vec<Ticket> {
        let mut result: Vec<Ticket> = Vec::new();
        result.push(self.my_ticket.clone());
        for ticket in self.tickets.iter() {
            if ticket.numbers.iter().all(|x| self.is_value_valid_in_any_rule(*x)) {
                result.push(ticket.clone());
            }
        }
        return result;
    }

    fn find_possible_rules(&self, tickets: &Vec<Ticket>, field_idx: usize) -> HashSet<usize> {
        let mut possible_rules: HashSet<usize> = (0..self.rules.len()).collect();
        for ticket in tickets.iter() {
            let number = ticket.numbers[field_idx];
            for rule_idx in 0..self.rules.len() {
                if !self.rules[rule_idx].accepts(number) {
                    possible_rules.remove(&rule_idx);
                }
            }
        }
        return possible_rules;
    }

    fn task2(&self) -> usize {
        let tickets = self.get_all_valid_tickets(); // includes my_ticket
        let mut possible_rules_for_field: Vec<HashSet<usize>> = Vec::new();
        let field_count = self.rules.len(); // == rule_count

        // Init
        for field_idx in 0..field_count {
            possible_rules_for_field.push(self.find_possible_rules(&tickets, field_idx));
        }

        // Iterate
        let mut used_rule_idxs: HashSet<usize> = HashSet::new();
        while used_rule_idxs.len() < field_count {
            for field_idx in 0..field_count {
                if possible_rules_for_field[field_idx].len() == 1 {
                    used_rule_idxs.insert(*possible_rules_for_field[field_idx].iter().next().unwrap());
                } else {
                    possible_rules_for_field[field_idx] = possible_rules_for_field[field_idx].difference(&used_rule_idxs).map(|x| *x).collect();
                }
            }
        }

        // Print result
        let mut result = 1;
        for field_idx in 0..field_count {
            let rule_idx : usize = *possible_rules_for_field[field_idx].iter().next().unwrap();
            println!("'{}' has index {} and value {}", self.rules[rule_idx].name, field_idx, self.my_ticket.numbers[field_idx]);
            if self.rules[rule_idx].name.starts_with("departure") {
                result *= self.my_ticket.numbers[field_idx];
            }
        }
        println!("result: {}", result);

        return 0;
    }
}

fn main() {
    let input = Input::new();
    input.dump();
    println!("task1: {}", input.task1());
    println!("task2: {}", input.task2());
}
