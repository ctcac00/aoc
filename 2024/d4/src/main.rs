use std::{collections::HashMap, fs::read_to_string};

fn is_valid(puzzle: &[Vec<char>], x: i32, y: i32) -> bool {
    x >= 0 && y >= 0 && x < puzzle.len() as i32 && y < puzzle[x as usize].len() as i32
}

fn word_search_p1(puzzle: &[Vec<char>], word: &str) {
    let directions = [
        (0, 1),
        (1, 0),
        (1, 1),
        (-1, 0),
        (0, -1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ];
    let mut results = HashMap::new();
    for x in 0..puzzle.len() {
        for y in 0..puzzle[x].len() {
            for (dx, dy) in directions.iter() {
                let mut found = true;
                for i in 0..word.len() {
                    let new_x = x as i32 + dx * i as i32;
                    let new_y = y as i32 + dy * i as i32;
                    if !is_valid(puzzle, new_x, new_y)
                        || puzzle[new_x as usize][new_y as usize] != word.chars().nth(i).unwrap()
                    {
                        found = false;
                        break;
                    }
                }
                if found {
                    results
                        .entry((x, y))
                        .and_modify(|val| *val += 1)
                        .or_insert(1);
                }
            }
        }
    }
    println!("{:?}", results.iter().fold(0, |acc, (_, v)| acc + v));
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let mut puzzle: Vec<Vec<char>> = Vec::new();
    input.lines().for_each(|line| {
        puzzle.push(line.chars().collect());
    });

    let word = &"XMAS";
    word_search_p1(&puzzle, word);
}
