use std::io::{self, BufRead};

struct Input {
    earliest_time: usize,
    bus_ids: Vec<usize>,
}

fn parse() -> Input {
    let stdin = io::stdin();
    let mut line_iter = stdin.lock().lines();

    let earliest_time: usize = line_iter.next().unwrap().unwrap().parse().unwrap();

    let bus_ids_str: String = line_iter.next().unwrap().unwrap();
    let bus_id_vec: Vec<String> = bus_ids_str.split(",").map(|x| x.to_string()).collect();
    let mut bus_ids: Vec<usize> = Vec::new();
    for iter in bus_id_vec.iter() {
        let num = iter.to_string();
        if num == "x" {
            bus_ids.push(0);
        } else {
            bus_ids.push(num.parse().unwrap());
        }
    }

    return Input { earliest_time: earliest_time, bus_ids: bus_ids };
}

fn find_best_bus_id(input: &Input) -> usize {
    let mut best_bus_id: usize = 0;
    let mut best_wait_time: usize = 99999999; // let's go !!
    for bus_id in input.bus_ids.iter() {
        if *bus_id == 0 {
            continue;
        }

        let factor_f = (input.earliest_time as f64 / *bus_id as f64).ceil();
        let factor = factor_f as usize;
        let next_time = bus_id * factor;
        assert!(next_time >= input.earliest_time);
        let wait_time = next_time - input.earliest_time;
        if best_wait_time > wait_time {
            best_wait_time = wait_time;
            best_bus_id = *bus_id;
        }
    }

    println!("best_bus_id: {}, best_wait_time: {}, hash: {}", best_bus_id, best_wait_time, best_bus_id * best_wait_time);
    return best_bus_id;
}

struct State {
    starts: Vec<u64>,
    adds: Vec<u64>,
    next: usize,
    ref_idx: usize,
}

fn get_next_valid_index(input: &Input, state: &mut State) -> usize {
    for i in state.next..input.bus_ids.len() {
        if input.bus_ids[i] != 0 {
            state.next = i + 1;
            return i;
        }
    }
    panic!("had no more indexes");
}

fn print_state(input: &Input, state: &State) {
    for i in 0..state.starts.len() {
        if input.bus_ids[i] == 0 {
            println!("[{}]: -", i);
        } else if state.starts[i] == 0 {
            println!("[{}]: ...", i);
        } else {
            let res = state.starts[i] * input.bus_ids[i] as u64 - i as u64;
            println!("[{}]: ({} + i * {}) * {} - {} = {}", i, state.starts[i], state.adds[i], input.bus_ids[i], i, res);
        }
    }
}

fn boot_strap(input: &Input, state: &mut State) {
    // Figure out indexes
    let a_idx = get_next_valid_index(input, state);
    let a_id = input.bus_ids[a_idx] as u64;
    state.ref_idx = a_idx;
    let b_idx = get_next_valid_index(input, state);
    let b_id = input.bus_ids[b_idx] as u64;
    println!("Indexes: [{}]{}, [{}]{}", a_idx, a_id, b_idx, b_id);

    // Figure out starts
    let mut a_start = 1u64;
    let mut b_start = 1u64;
    loop {
        let a_time = a_id * a_start - a_idx as u64;
        let b_time = b_id * b_start - b_idx as u64;
        if a_time == b_time {
            break;
        }
        if a_time < b_time {
            a_start += 1;
        } else {
            b_start += 1;
        }
    }
    println!("Starting positions: {} {}", a_start, b_start);

    // Figure out adds
    let mut a_add = a_start + 1;
    let mut b_add = b_start + 1;
    loop {
        let a_time = a_id * a_add - a_idx as u64;
        let b_time = b_id * b_add - b_idx as u64;
        if a_time == b_time {
            break;
        }
        if a_time < b_time {
            a_add += 1;
        } else {
            b_add += 1;
        }
    }
    a_add = a_add - a_start;
    b_add = b_add - b_start;
    println!("Additions: {} {}", a_add, b_add);
    println!("{} == {} + 1", solve(input, state, a_idx, 8), solve(input, state, b_idx, 8));

    // Save info in state
    state.starts[a_idx] = a_start;
    state.starts[b_idx] = b_start;
    state.adds[a_idx] = a_add;
    state.adds[b_idx] = b_add;
}

fn solve(input: &Input, state: &State, idx: usize, fac: u64) -> u64 {
    (state.starts[idx] + fac * state.adds[idx]) * input.bus_ids[idx] as u64 - idx as u64
}

fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    for a in 2..n {
        if n % a == 0 {
            return false; // if it is not the last statement you need to use `return`
        }
    }
    true // last value to return
}

fn do_next_guy(input: &Input, state: &mut State) {
    let ref_idx = state.ref_idx;
    let ref_id = input.bus_ids[ref_idx] as u64;
    let ref_start = state.starts[ref_idx];
    let ref_add = state.adds[ref_idx];
    let next_idx = get_next_valid_index(input, state);
    let next_id = input.bus_ids[next_idx] as u64;
    assert!(is_prime(next_id));
    assert!(ref_idx < input.bus_ids.len());
    assert!(next_idx < input.bus_ids.len());

    println!("Go time with: [{}]: {}", next_idx, next_id);

    // Find start
    let mut fac = 0;
    state.starts[next_idx] = solve(input, state, ref_idx, fac) / next_id;
    while state.starts[next_idx] * next_id < next_idx as u64 {
        state.starts[next_idx] += 1;
    }
    loop {
        let needed = solve(input, state, ref_idx, fac);
        let have = solve(input, state, next_idx, 0);
        if needed == have {
            println!("Found solution: {} with fac: {}", needed, fac);
            break;
        }
        if needed > have {
            state.starts[next_idx] += 1;
        } else {
            fac += 1;
            state.starts[next_idx] = solve(input, state, ref_idx, fac) / next_id;
        }
    }

    // Find adds
    let next_start = state.starts[next_idx];
    state.starts[next_idx] += 1;
    loop {
        let needed = solve(input, state, ref_idx, fac);
        let have = solve(input, state, next_idx, 0);
        if needed == have {
            println!("Found add solution: {} with fac: {}", needed, fac);
            break;
        }
        if needed > have {
            state.starts[next_idx] += 1;
        } else {
            fac += 1;
            state.starts[next_idx] = solve(input, state, ref_idx, fac) / next_id;
        }
    }
    state.adds[next_idx] = state.starts[next_idx] - next_start;
    state.starts[next_idx] = next_start;

    // Update state
    // for i in 0..(state.next - 1) {
    //     // state.starts[i] += fac * state.adds[i]; // Factor as new start
    //     state.adds[next_idx] *= state.adds[i]; // New add is the product of all primes before
    // }
    state.ref_idx = next_idx;
}

#[allow(dead_code)]
fn task1() {
    let input = parse();
    find_best_bus_id(&input);
}

#[allow(dead_code)]
fn task2() {
    let input = parse();
    let mut state = State { starts: Vec::new(), adds: Vec::new(), next: 0, ref_idx: 0 };
    state.starts = vec![0; input.bus_ids.len()];
    state.adds = vec![1; input.bus_ids.len()];
    boot_strap(&input, &mut state);
    print_state(&input, &mut state);
    while state.next != input.bus_ids.len() {
        do_next_guy(&input, &mut state);
        print_state(&input, &mut state);
    }

    // println!("earliest_time: {}", input.earliest_time);
    // for iter in input.bus_ids.iter() {
    //     if *iter != 0 {
    //         println!("ids: {} -> {}", iter, is_prime(*iter));
    //     }
    // }
}

fn main() {
// task1();
    task2();
}
