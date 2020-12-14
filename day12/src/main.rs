use std::io::{self, BufRead};

struct Command {
    kind: char,
    num: isize,
}

struct State {
    x: isize,
    y: isize,
    dir: isize,
}

struct StateTask2 {
    x: isize,
    y: isize,
    way_x: isize,
    way_y: isize,
}

fn parse() -> Vec<Command> {
    let mut result: Vec<Command> = Vec::new();
    let stdin = io::stdin();
    for strange_line in stdin.lock().lines() {
        let line = strange_line.unwrap();
        let mut iter = line.chars();
        let kind: char = iter.next().unwrap();
        let num_str: String = iter.collect();
        let num: isize = num_str.parse().unwrap();
        let command = Command { kind: kind, num: num };
        result.push(command);
    }
    return result;
}

fn process_commands_task_1(state: &mut State, commands: &Vec<Command>) {
    for command in commands.iter() {
        match command.kind {
            'N' => { state.y += command.num; }
            'E' => { state.x += command.num; }
            'S' => { state.y -= command.num; }
            'W' => { state.x -= command.num; }
            'R' => { state.dir = (state.dir - (command.num % 360) + 360) % 360; }
            'L' => { state.dir = (state.dir + (command.num % 360) + 360) % 360; }
            'F' => {
                match state.dir {
                    0 => { state.x += command.num; }
                    90 => { state.y += command.num; }
                    180 => { state.x -= command.num; }
                    270 => { state.y -= command.num; }
                    any => { panic!("unknown direction!!!!: '{}'", any); }
                }
            }
            any => { panic!("unknown command!!!!: '{}'", any); }
        }
    }
}

fn rotate_left(state: &mut StateTask2, rot: isize) {
    let mut rot = rot;
    while rot > 0 {
        let new_x = -state.way_y;
        let new_y = state.way_x;
        state.way_x = new_x;
        state.way_y = new_y;
        rot -= 90;
    }
}

fn process_commands_task_2(state: &mut StateTask2, commands: &Vec<Command>) {
    for command in commands.iter() {
        match command.kind {
            'N' => { state.way_y += command.num; }
            'E' => { state.way_x += command.num; }
            'S' => { state.way_y -= command.num; }
            'W' => { state.way_x -= command.num; }
            'R' => { rotate_left(state, (-command.num + 360) % 360); }
            'L' => { rotate_left(state, command.num); }
            'F' => {
                state.x += state.way_x * command.num;
                state.y += state.way_y * command.num;
            }
            any => { panic!("unknown command!!!!: '{}'", any); }
        }
    }
}

#[allow(dead_code)]
fn task1() {
    let commands = parse();
    let mut state = State { x: 0, y: 0, dir: 0 };
    process_commands_task_1(&mut state, &commands);

    println!("x: {}, y: {}, dir: {}", state.x, state.y, state.dir);
}

#[allow(dead_code)]
fn task2() {
    let commands = parse();
    let mut state = StateTask2 { x: 0, y: 0, way_x: 10, way_y: 1 };
    process_commands_task_2(&mut state, &commands);

    println!("x: {}, y: {}", state.x, state.y);
    println!("way_x: {}, way_y: {}", state.way_x, state.way_y);
}

fn main() {
    task2();
}
