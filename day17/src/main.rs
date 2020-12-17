use std::io::{self, BufRead};
use std::collections::HashSet;
use std::collections::HashMap;

#[derive(Copy, Clone, Eq, Hash, PartialEq, Debug)]
struct Cube(isize, isize, isize);

struct State {
    active_cubes: HashSet<Cube>,
}

impl State {
    fn new() -> State {
        // Parse and store coordinates of all active cubes in a map: (x, y, z)
        let mut active_cubes: HashSet<Cube> = HashSet::new();
        let stdin = io::stdin();
        for (y, strange_line) in stdin.lock().lines().enumerate() {
            let line = strange_line.unwrap();
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    active_cubes.insert(Cube(x as isize, y as isize, 0));
                }
            }
        }
        return State { active_cubes };
    }

    fn iter(&self) -> State {
        // Calculate the number of active neighbours for every position in space
        let mut neighbor_counts: HashMap<Cube, usize> = HashMap::new();
        for cube in &self.active_cubes {
            for x in -1..2 {
                for y in -1..2 {
                    for z in -1..2 {
                        if !(x == 0 && y == 0 && z == 0) {
                            let stat = neighbor_counts.entry(Cube(cube.0 + x, cube.1 + y, cube.2 + z)).or_insert(0);
                            *stat += 1;
                        }
                    }
                }
            }
        }

        // Iterate over all positions and decide according to the rules if there should be an active block
        let mut active_cubes: HashSet<Cube> = HashSet::new();
        for (cube, count) in neighbor_counts {
            // If a cube is active and exactly 2 or 3 of its neighbors are also active, the cube remains active. Otherwise, the cube becomes inactive.
            if self.active_cubes.contains(&cube) && (count == 2 || count == 3) {
                active_cubes.insert(cube);
            }

            // If a cube is inactive but exactly 3 of its neighbors are active, the cube becomes active. Otherwise, the cube remains inactive.
            if !self.active_cubes.contains(&cube) && count == 3 {
                active_cubes.insert(cube);
            }
        }

        return State { active_cubes };
    }

    fn count_active_cubes(&self) -> usize { self.active_cubes.len() }
}


#[derive(Copy, Clone, Eq, Hash, PartialEq, Debug)]
struct Cube2(isize, isize, isize, isize);

struct State2 {
    active_cubes: HashSet<Cube2>,
}

impl State2 {
    fn new() -> State2 {
        // Parse and store coordinates of all active cubes in a map: (x, y, z)
        let mut active_cubes: HashSet<Cube2> = HashSet::new();
        let stdin = io::stdin();
        for (y, strange_line) in stdin.lock().lines().enumerate() {
            let line = strange_line.unwrap();
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    active_cubes.insert(Cube2(x as isize, y as isize, 0, 0));
                }
            }
        }
        return State2{ active_cubes };
    }

    fn iter(&self) -> State2 {
        // Calculate the number of active neighbours for every position in space
        let mut neighbor_counts: HashMap<Cube2, usize> = HashMap::new();
        for cube in &self.active_cubes {
            for x in -1..2 {
                for y in -1..2 {
                    for z in -1..2 {
                        for w in -1..2 {
                            if !(x == 0 && y == 0 && z == 0 && w == 0) {
                                let stat = neighbor_counts.entry(Cube2(cube.0 + x, cube.1 + y, cube.2 + z, cube.3 + w)).or_insert(0);
                                *stat += 1;
                            }
                        }
                    }
                }
            }
        }

        // Iterate over all positions and decide according to the rules if there should be an active block
        let mut active_cubes: HashSet<Cube2> = HashSet::new();
        for (cube, count) in neighbor_counts {
            // If a cube is active and exactly 2 or 3 of its neighbors are also active, the cube remains active. Otherwise, the cube becomes inactive.
            if self.active_cubes.contains(&cube) && (count == 2 || count == 3) {
                active_cubes.insert(cube);
            }

            // If a cube is inactive but exactly 3 of its neighbors are active, the cube becomes active. Otherwise, the cube remains inactive.
            if !self.active_cubes.contains(&cube) && count == 3 {
                active_cubes.insert(cube);
            }
        }

        return State2 { active_cubes };
    }

    fn count_active_cubes(&self) -> usize { self.active_cubes.len() }
}

fn main() {
    // let mut state = State::new();
    // println!("[0] #cubes: {}", state.count_active_cubes());
    // let mut state = state.iter();
    // println!("[1] #cubes: {}", state.count_active_cubes());
    // let mut state = state.iter();
    // println!("[2] #cubes: {}", state.count_active_cubes());
    // let mut state = state.iter();
    // println!("[3] #cubes: {}", state.count_active_cubes());
    // let mut state = state.iter();
    // println!("[4] #cubes: {}", state.count_active_cubes());
    // let mut state = state.iter();
    // println!("[5] #cubes: {}", state.count_active_cubes());
    // let mut state = state.iter();
    // println!("[6] #cubes: {}", state.count_active_cubes());

    let mut state = State2::new();
    println!("[0] #cubes: {}", state.count_active_cubes());
    let mut state = state.iter();
    println!("[1] #cubes: {}", state.count_active_cubes());
    let mut state = state.iter();
    println!("[2] #cubes: {}", state.count_active_cubes());
    let mut state = state.iter();
    println!("[3] #cubes: {}", state.count_active_cubes());
    let mut state = state.iter();
    println!("[4] #cubes: {}", state.count_active_cubes());
    let mut state = state.iter();
    println!("[5] #cubes: {}", state.count_active_cubes());
    let mut state = state.iter();
    println!("[6] #cubes: {}", state.count_active_cubes());
}
