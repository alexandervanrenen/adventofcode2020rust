use std::io::{self, BufRead};

#[derive(Copy, Clone, PartialEq)]
enum FieldType {
    Tree,
    Grass,
}

struct Map {
    map: Vec<FieldType>,
    width: usize,
    height: usize,
}

impl Map {
    fn get(&self, x: usize, y: usize) -> FieldType {
        assert!(y < self.height);
        self.map[y * self.width + x % self.width]
    }

    fn parse(&mut self) {
        let char_to_field_type = |ele| match ele {
            '#' => FieldType::Tree,
            '.' => FieldType::Grass,
            _ => panic!("crash and burn"),
        };

        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let mut res: Vec<FieldType> = line.unwrap().chars().map(char_to_field_type).collect();
            self.width = res.len();
            self.height = self.height + 1;
            self.map.append(&mut res);
        }
    }

    fn dump(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                match self.get(x, y) {
                    FieldType::Tree => print!("#"),
                    FieldType::Grass => print!("."),
                }
            }
            println!();
        }
    }

    fn slay_ride(&self, right: usize, down: usize) -> usize {
        let mut pos_x = 0;
        let mut pos_y = 0;
        let mut hit_tree_count = 0;

        while pos_y < self.height {
            if self.get(pos_x, pos_y) == FieldType::Tree {
                hit_tree_count += 1;
            }

            pos_x += right;
            pos_y += down;
        }

        return hit_tree_count;
    }
}

fn main() {
    let mut map: Map = Map { width: 0, height: 0, map: Vec::new() };
    map.parse();
    map.dump();

    // Task 1
    let task_1_result = map.slay_ride(3, 1);
    println!("Task 1 hit tree count: {}", task_1_result);

    // Task 2
    let directions = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut task_2_result = 1;
    for direction in directions.iter() {
        task_2_result *= map.slay_ride(direction.0, direction.1);
    }
    println!("Task 2 hit tree count: {}", task_2_result);
}
