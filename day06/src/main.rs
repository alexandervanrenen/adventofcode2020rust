use std::io::{self, Read};

fn task1() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let mut sum = 0;

    for line in buffer.split("\n\n") {
        let mut found = vec![false; 26];
        for c in line.chars().filter(|c| 'a' <= *c && *c <= 'z') {
            let idx = c as usize - 'a' as usize;
            found[idx] = true;
        }

        let yes_answers = found.iter().filter(|e| **e).count();
        sum += yes_answers;
        println!("'Yes' answers in group: {}", yes_answers);
    }

    println!("Result: {}", sum);

    Ok(())
}

fn task2() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    // Remove the last new line, because we use new lines to count people.
    // The last one is not removed as the other ones by the split("\n\n")
    buffer.pop();

    let mut sum = 0;

    for line in buffer.split("\n\n") {
        let mut found = vec![0; 26];
        let mut people_in_group = 1;
        for c in line.chars() {
            if 'a' <= c && c <= 'z' {
                let idx = c as usize - 'a' as usize;
                found[idx] += 1;
            } else {
                people_in_group += 1;
            }
        }

        let yes_answers = found.iter().filter(|e| **e == people_in_group).count();
        sum += yes_answers;
        println!("'Yes' answers in {} people group: {}", people_in_group, yes_answers);
    }

    println!("Result: {}", sum);

    Ok(())
}

fn main() -> io::Result<()> {
    // task1()
    task2()
}
