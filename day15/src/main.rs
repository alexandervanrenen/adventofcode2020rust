use std::collections::HashMap;

struct Record(usize, usize);

fn add_to_history(history: &mut Vec<Record>, last_used_index: &mut HashMap<usize, usize>, num: usize) {
    if let Some(x) = last_used_index.get_mut(&num) {
        history.push(Record(num, *x));
        *x = history.len() - 1;
    } else {
        history.push(Record(num, history.len()));
        last_used_index.insert(num, history.len() - 1);
    }
}

fn taskWithHashMap(numbers: &Vec<usize>) {
    let mut history: Vec<Record> = Vec::new();
    history.reserve(30000001);
    let mut last_used_index: HashMap<usize, usize> = HashMap::new();

    // Init with given numbers
    for iter in numbers.iter() {
        add_to_history(&mut history, &mut last_used_index, *iter);
    }

    while history.len() <= 30000000 {
        let last_record = history.last().unwrap();
        let last_num: usize = last_record.0;
        let last_idx: usize = last_record.1;

        if last_idx == history.len() - 1 {
            // Never seen ?
            add_to_history(&mut history, &mut last_used_index, 0);
        } else {
            // Seen before at last_idx
            let new = history.len() - 1 - last_idx;
            add_to_history(&mut history, &mut last_used_index, new);
        }
    }

    println!("task1: {}", history[2020 - 1].0);
    println!("task2: {}", history[30000000 - 1].0);
}

fn add_to_history2(history: &mut Vec<Record>, last_used_index: &mut Vec<isize>, num: usize) {
    let idx = last_used_index[num];
    if idx != -1 {
        history.push(Record(num, idx as usize));
    } else {
        history.push(Record(num, history.len()));
    }
    last_used_index[num] = (history.len() - 1) as isize;
}

fn taskWithVector(numbers: &Vec<usize>) {
    let mut history: Vec<Record> = Vec::new();
    history.reserve(30000001);
    let mut last_used_index: Vec<isize> = Vec::new();
    last_used_index.resize(29421065, -1);

    // Init with given numbers
    for iter in numbers.iter() {
        add_to_history2(&mut history, &mut last_used_index, *iter);
    }

    while history.len() <= 30000000 {
        let last_record = history.last().unwrap();
        let last_num: usize = last_record.0;
        let last_idx: usize = last_record.1;

        if last_idx == history.len() - 1 {
            // Never seen ?
            add_to_history2(&mut history, &mut last_used_index, 0);
        } else {
            // Seen before at last_idx
            let new = history.len() - 1 - last_idx;
            add_to_history2(&mut history, &mut last_used_index, new);
        }
    }

    println!("task1: {}", history[2020 - 1].0);
    println!("task2: {}", history[30000000 - 1].0);
}

fn main() {
    let mut x = vec![0, 3, 6];
    let mut x = vec![1, 3, 2];
    let mut x = vec![2, 1, 3];
    let mut x = vec![1, 2, 3];
    let mut x = vec![2, 3, 1];
    let mut x = vec![3, 2, 1];
    let mut x = vec![3, 1, 2];
    let mut x = vec![19, 20, 14, 0, 9, 1];
    taskWithVector(&x);
    // taskWithHashMap(vec![19, 20, 14, 0, 9, 1]);
}
