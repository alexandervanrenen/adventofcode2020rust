use std::io::{self, BufRead};

struct SeatMap {
    seats: Vec<char>,
    width: isize,
    height: isize,
}

impl SeatMap {
    fn new() -> SeatMap {
        SeatMap { seats: Vec::new(), width: 0, height: 0 }
    }

    fn parse(&mut self) {
        let stdin = io::stdin();
        for strange_line in stdin.lock().lines() {
            for c in strange_line.unwrap().chars() {
                self.seats.push(c);
            }
            self.height += 1;
        }
        self.width = self.seats.len() as isize / self.height;
    }

    fn get(&self, x: isize, y: isize) -> char {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            '.'
        } else {
            self.seats[(y * self.width + x) as usize]
        }
    }

    fn get_occupied_neighbor_count(&self, x: isize, y: isize) -> usize {
        (if self.get(x - 1, y - 1) == '#' { 1 } else { 0 })
            + (if self.get(x + 0, y - 1) == '#' { 1 } else { 0 })
            + (if self.get(x + 1, y - 1) == '#' { 1 } else { 0 })
            + (if self.get(x - 1, y + 0) == '#' { 1 } else { 0 })
            + (if self.get(x + 1, y + 0) == '#' { 1 } else { 0 })
            + (if self.get(x - 1, y + 1) == '#' { 1 } else { 0 })
            + (if self.get(x + 0, y + 1) == '#' { 1 } else { 0 })
            + (if self.get(x + 1, y + 1) == '#' { 1 } else { 0 })
    }

    fn has_occupied_seat_in_direction(&self, x: isize, y: isize, dx: isize, dy: isize) -> bool {
        let mut x = x;
        let mut y = y;
        loop {
            x += dx;
            y += dy;
            if x < 0 || y < 0 || x >= self.width || y >= self.height { return false; }
            if self.get(x, y) == 'L' { return false; }
            if self.get(x, y) == '#' { return true; }
        }
    }

    fn get_occupied_neighbor_count_task2(&self, x: isize, y: isize) -> usize {
        (if self.has_occupied_seat_in_direction(x, y, -1, -1) { 1 } else { 0 })
            + (if self.has_occupied_seat_in_direction(x, y, 0, -1) { 1 } else { 0 })
            + (if self.has_occupied_seat_in_direction(x, y, 1, -1) { 1 } else { 0 })
            + (if self.has_occupied_seat_in_direction(x, y, -1, 0) { 1 } else { 0 })
            + (if self.has_occupied_seat_in_direction(x, y, 1, 0) { 1 } else { 0 })
            + (if self.has_occupied_seat_in_direction(x, y, -1, 1) { 1 } else { 0 })
            + (if self.has_occupied_seat_in_direction(x, y, 0, 1) { 1 } else { 0 })
            + (if self.has_occupied_seat_in_direction(x, y, 1, 1) { 1 } else { 0 })
    }

    fn dump(&self) {
        println!("w: {}, h: {}", self.width, self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self.get(x, y));
            }
            println!();
        }
    }

    fn iterate(&mut self, map: &SeatMap) {
        self.width = map.width;
        self.height = map.height;
        for y in 0..map.height {
            for x in 0..map.width {
                match map.get(x, y) {
                    'L' => self.seats.push(if map.get_occupied_neighbor_count(x, y) == 0 { '#' } else { 'L' }),
                    '#' => self.seats.push(if map.get_occupied_neighbor_count(x, y) >= 4 { 'L' } else { '#' }),
                    x => self.seats.push(x),
                }
            }
        }
    }

    fn iterate_task2(&mut self, map: &SeatMap) {
        self.width = map.width;
        self.height = map.height;
        for y in 0..map.height {
            for x in 0..map.width {
                match map.get(x, y) {
                    'L' => self.seats.push(if map.get_occupied_neighbor_count_task2(x, y) == 0 { '#' } else { 'L' }),
                    '#' => self.seats.push(if map.get_occupied_neighbor_count_task2(x, y) >= 5 { 'L' } else { '#' }),
                    x => self.seats.push(x),
                }
            }
        }
    }

    fn is_same(&self, map: &SeatMap) -> bool {
        for y in 0..map.height {
            for x in 0..map.width {
                if self.get(x, y) != map.get(x, y) {
                    return false;
                }
            }
        }
        return true;
    }

    fn count_occupied_seats(&self) -> usize {
        self.seats.iter().filter(|e| **e == '#').count()
    }
}

#[allow(dead_code)]
fn task1() {
    let mut seats = SeatMap::new();
    seats.parse();
    seats.dump();
    loop {
        let mut seats_n1 = SeatMap::new();
        seats_n1.iterate(&seats);
        seats_n1.dump();
        if seats.is_same(&seats_n1) {
            break;
        } else {
            seats = seats_n1;
        }
    }
    println!("{}", seats.count_occupied_seats());
}

#[allow(dead_code)]
fn task2() {
    let mut seats = SeatMap::new();
    seats.parse();
    seats.dump();
    loop {
        let mut seats_n1 = SeatMap::new();
        seats_n1.iterate_task2(&seats);
        seats_n1.dump();
        if seats.is_same(&seats_n1) {
            break;
        } else {
            seats = seats_n1;
        }
    }
    println!("{}", seats.count_occupied_seats());
}

fn main() {
    // task1();
    task2();
}

