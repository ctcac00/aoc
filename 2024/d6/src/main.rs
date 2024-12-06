use std::{collections::HashSet, fs::read_to_string};

fn print_map(map: &[Vec<char>]) {
    for row in map {
        println!("{:?}", row);
    }
    println!();
}

fn is_valid(map: &[Vec<char>], x: i32, y: i32) -> bool {
    x >= 0 && x < map.len() as i32 && y >= 0 && y < map[0].len() as i32
}

fn change_direction(c: char) -> char {
    match c {
        '^' => '>',
        'v' => '<',
        '>' => 'v',
        '<' => '^',
        _ => panic!("Invalid move"),
    }
}

fn get_move(c: char) -> (i32, i32) {
    match c {
        '^' => (-1, 0),
        'v' => (1, 0),
        '>' => (0, 1),
        '<' => (0, -1),
        _ => panic!("Invalid move"),
    }
}

fn move_guard(
    mut map: Vec<Vec<char>>,
    x: usize,
    y: usize,
    visited: &mut HashSet<(usize, usize)>,
) -> (usize, usize) {
    visited.insert((x, y));
    let (dx, dy) = get_move(map[x][y]);
    let (nx, ny) = (x as i32 + dx, y as i32 + dy);
    if !is_valid(&map, nx, ny) {
        (x, y)
    } else if map[nx as usize][ny as usize] == '.' {
        map[nx as usize][ny as usize] = map[x][y];
        map[x][y] = '.';
        return move_guard(map, nx as usize, ny as usize, visited);
    } else {
        // need to change direction to 90 degrees
        let new_dir = change_direction(map[x][y]);
        map[x][y] = new_dir;
        return move_guard(map, x, y, visited);
    }
}

fn find_starting_point(map: &[Vec<char>]) -> (usize, usize) {
    map.iter()
        .enumerate()
        .find_map(|(i, row)| {
            row.iter().enumerate().find_map(|(j, &c)| {
                if c == '^' || c == 'v' || c == '>' || c == '<' {
                    Some((i, j))
                } else {
                    None
                }
            })
        })
        .unwrap()
}

fn solve_p1(map: Vec<Vec<char>>) -> i32 {
    let mut visited = HashSet::new();
    let (x, y) = find_starting_point(&map);
    move_guard(map, x, y, &mut visited);

    visited.len() as i32
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    println!("{:?}", solve_p1(map));
}
