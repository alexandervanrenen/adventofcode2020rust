use std::io::{self, Read};
use std::collections::HashSet;
use regex::Regex;

// Part 1 ------------------------------------------------------------------------------------------

#[allow(dead_code)]
fn is_password_valid(pw: &String) -> bool {
    let required_keys: HashSet<String> = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"].iter().map(|x| x.to_string()).collect();
    let found_keys: HashSet<String> = pw.split(" ").map(|prop| prop.to_string().chars().take(3).collect()).collect();
    required_keys.difference(&found_keys).count() == 0
}

#[allow(dead_code)]
fn task_1_count() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let pws = buffer.split("\n\n").map(|str| str.to_string().replace("\n", " ").trim().to_string());
    let count = pws.filter(|pw| is_password_valid(pw)).count();
    println!("Valid password count {}", count);
    Ok(())
}

// Part 2 ------------------------------------------------------------------------------------------

#[derive(Default)]
struct Passport {
    birth_year: Option<String>,
    issue_year: Option<String>,
    expiration_year: Option<String>,
    height: Option<String>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    passport_id: Option<String>,
    country_id: Option<String>,
}

impl Passport {
    fn new(str: &String) -> Passport {
        let mut result = Passport::default();

        for property in str.replace("\n", " ").trim().split(' ').map(String::from) {
            let arr: Vec<&str> = property.split(':').collect();
            if arr.len() != 2 {
                println!("Invalid format: '{}' -> '{}'", str, property);
            }
            let key = arr[0];
            let value = arr[1];
            match key {
                "byr" => result.birth_year = Some(value.to_string()),
                "iyr" => result.issue_year = Some(value.to_string()),
                "eyr" => result.expiration_year = Some(value.to_string()),
                "hgt" => result.height = Some(value.to_string()),
                "hcl" => result.hair_color = Some(value.to_string()),
                "ecl" => result.eye_color = Some(value.to_string()),
                "pid" => result.passport_id = Some(value.to_string()),
                "cid" => result.country_id = Some(value.to_string()),
                _ => panic!("Invalid passport format...")
            }
        }

        return result;
    }

    fn has_all_required_fields(&self) -> bool {
        self.birth_year.is_some()
            && self.issue_year.is_some()
            && self.expiration_year.is_some()
            && self.height.is_some()
            && self.hair_color.is_some()
            && self.eye_color.is_some()
            && self.passport_id.is_some()
        // && self.country_id.is_some() // Ignore this one, to get us through passport control :)
    }

    // Not _very_ fast code ;)
    fn is_valid(&self) -> bool {
        if !self.has_all_required_fields() { return false; }

        if !(1920..2003isize).contains(&self.birth_year.as_ref().unwrap().parse().unwrap()) { return false; }
        if !(2010..2021isize).contains(&self.issue_year.as_ref().unwrap().parse().unwrap()) { return false; }
        if !(2020..2031isize).contains(&self.expiration_year.as_ref().unwrap().parse().unwrap()) { return false; }

        let height: &String = self.height.as_ref().unwrap();
        if height.ends_with("cm") {
            let num: usize = height.replace("cm", "").parse().unwrap();
            if !(150..194).contains(&num) { return false; }
        } else {
            let num: usize = height.replace("in", "").parse().unwrap();
            if !(59..77).contains(&num) { return false; }
        }

        let re = Regex::new(r"^#([0-9]|[a-f]){6}$").unwrap();
        if !re.is_match(self.hair_color.as_ref().unwrap()) { return false; }

        let re = Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$").unwrap();
        if !re.is_match(self.eye_color.as_ref().unwrap()) { return false; }

        let re = Regex::new(r"^[0-9]{9}$").unwrap();
        if !re.is_match(self.passport_id.as_ref().unwrap()) { return false; }

        return true;
    }
}

fn parse_passports() -> io::Result<Vec<Passport>> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let pw_iter = buffer.split("\n\n").map(|pw_str| Passport::new(&pw_str.to_string()));
    Ok(pw_iter.collect())
}

fn task_2_validation() -> io::Result<()> {
    let passports = parse_passports()?;
    let valid_password_count = passports.iter().filter(|pw| pw.is_valid()).count();
    println!("Valid password count: {}", valid_password_count);
    Ok(())
}

fn main() -> io::Result<()> {
    // task_1_count()
    task_2_validation()
}
