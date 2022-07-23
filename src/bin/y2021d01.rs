use aoc::*;

fn part1(input: &Vec<u32>) -> usize {
    input.windows(2).filter(|x| x[1] > x[0]).count()
}

fn part2(input: &Vec<u32>) -> usize {
    input.windows(4).filter(|x| x[3] > x[0]).count()
}

fn main() {
    let mut input: Input = Input::new();
    if let Err(e) = input.read_file("input/y2021d01.txt") {
        println!("{}", e);
        return;
    }

    let parsed: Vec<u32> = input.data.iter().map(|s| s.parse().unwrap()).collect();

    let increases = part1(&parsed);
    println!("Part 1 Increases: {}", increases);

    let increases = part2(&parsed);
    println!("Part 2 Increases: {}", increases);
}
