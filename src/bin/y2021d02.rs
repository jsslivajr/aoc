use aoc::*;

enum Direction {
    Up,
    Down,
    Forward,
    Unknown,
}

struct Order {
    direction: Direction,
    magnitude: i32,
}

impl Order {
    pub fn new(line: &str) -> Self {
        let mut iter = line.split_whitespace();
        let direction = match iter.next() {
            Some(s) => match s {
                "up" => Direction::Up,
                "down" => Direction::Down,
                "forward" => Direction::Forward,
                _ => Direction::Unknown,
            },
            None => Direction::Unknown,
        };

        let magnitude = match iter.next() {
            Some(s) => s.parse().unwrap(),
            None => 0,
        };

        Self {
            direction,
            magnitude,
        }
    }
}

fn part1(input: &Vec<Order>) -> i32 {
    let mut depth = 0;
    let mut horiz = 0;
    input.iter().for_each(|o| match o.direction {
        Direction::Up => depth -= o.magnitude,
        Direction::Down => depth += o.magnitude,
        Direction::Forward => horiz += o.magnitude,
        Direction::Unknown => {}
    });

    depth * horiz
}

fn part2(input: &Vec<Order>) -> i32 {
    let mut depth = 0;
    let mut horiz = 0;
    let mut aim = 0;

    input.iter().for_each(|o| match o.direction {
        Direction::Up => aim -= o.magnitude,
        Direction::Down => aim += o.magnitude,
        Direction::Forward => {
            horiz += o.magnitude;
            depth += aim * o.magnitude;
        },
        Direction::Unknown => {}
    });

    depth * horiz
}

fn main() {
    let mut input: Input = Input::new();
    if let Err(e) = input.read_file("input/y2021d02.txt") {
        println!("{}", e);
        return;
    }

    let parsed: Vec<Order> = input.data.iter().map(|x| Order::new(x)).collect();

    let result = part1(&parsed);
    println!("Part 1: {}", result);
    let result = part2(&parsed);
    println!("Part 2: {}", result);
}
