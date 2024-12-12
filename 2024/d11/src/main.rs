use std::fs::read_to_string;

fn has_even_digits(stone: u64) -> bool {
    let stone_str = stone.to_string();
    stone_str.len() % 2 == 0
}

fn split_stone(stone: u64) -> (u64, u64) {
    let stone_str = stone.to_string();
    let mid = stone_str.len() / 2;
    let (left, right) = stone_str.split_at(mid);
    (left.parse::<u64>().unwrap(), right.parse::<u64>().unwrap())
}

fn blink(stones: &mut Vec<u64>) {
    let mut i = 0;
    while i < stones.len() {
        if stones[i] == 0 {
            stones[i] = 1;
        } else if has_even_digits(stones[i]) {
            let (left, right) = split_stone(stones[i]);
            stones[i] = left;
            stones.insert(i + 1, right);
            i += 1;
        } else {
            stones[i] *= 2024;
        }
        i += 1;
    }
}

fn solve(stones: &mut Vec<u64>, i: i32) -> u64 {
    for _ in 0..i {
        blink(stones);
        // println!("{:?}", stones);
    }

    stones.len() as u64
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let stones = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<_>>()
        .concat();

    println!("{:?}", stones);
    println!("Part 1: {}", solve(&mut stones.clone(), 25));
}
