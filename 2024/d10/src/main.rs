use std::{collections::HashMap, fs::read_to_string};

const MOVES: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Pos {
    x: i32,
    y: i32,
}

fn find_neighbors(map: &HashMap<Pos, i32>, pos: &Pos) -> Vec<Pos> {
    MOVES
        .iter()
        .map(|(dx, dy)| Pos {
            x: pos.x + dx,
            y: pos.y + dy,
        })
        .filter(|p| map.contains_key(p) && *map.get(p).unwrap() == *map.get(pos).unwrap() + 1)
        .collect()
}

fn dfs(map: &HashMap<Pos, i32>, start: Pos, visited: &mut HashMap<Pos, i32>) {
    visited.insert(start, *map.get(&start).unwrap());
    // println!("Start {:?}", map.get_key_value(&start).unwrap());

    for neighbor in find_neighbors(map, &start) {
        if !visited.contains_key(&neighbor) {
            dfs(map, neighbor, visited);
        }
    }
}

fn dfs_v2(map: &HashMap<Pos, i32>, start: Pos, visited: &mut HashMap<Pos, i32>, count: &mut i32) {
    visited.insert(start, *map.get(&start).unwrap());
    // println!("Start {:?}", map.get_key_value(&start).unwrap());

    if *map.get(&start).unwrap() == 9 {
        *count += 1;
        // println!("Count -> {:?}", count);
    }

    for neighbor in find_neighbors(map, &start) {
        if !visited.contains_key(&neighbor) {
            dfs_v2(map, neighbor, visited, count);
        }
    }

    visited.remove(&start);
}

fn find_pos(map: &HashMap<Pos, i32>, val: i32) -> Vec<Pos> {
    map.iter()
        .filter(|&(_, &value)| value == val)
        .map(|(&key, _)| key)
        .collect()
}

fn solve_p1(map: &HashMap<Pos, i32>) -> i32 {
    let starts = find_pos(map, 0);
    // println!("starts -> {:?}", starts);
    let mut trailheads = 0;
    for s in starts {
        let mut visited = HashMap::new();
        dfs(map, s, &mut visited);
        trailheads += find_pos(&visited, 9).len() as i32;
    }

    trailheads
}

fn solve_p2(map: &HashMap<Pos, i32>) -> i32 {
    let starts = find_pos(map, 0);
    // println!("starts -> {:?}", starts);
    let mut count = 0;
    for s in starts {
        let mut visited = HashMap::new();
        dfs_v2(map, s, &mut visited, &mut count);
        // println!("visited -> {:?}", visited);
    }

    count
}
fn main() {
    let input = read_to_string("input.txt");

    let mut map = HashMap::new();

    for (x, line) in input.unwrap().lines().enumerate() {
        for (y, c) in line.chars().enumerate() {
            map.insert(
                Pos {
                    x: x as i32,
                    y: y as i32,
                },
                c.to_digit(10).unwrap() as i32,
            );
        }
    }

    // println!("{:?}", map);
    println!("P1 -> {:?}", solve_p1(&map));
    println!("P2 -> {:?}", solve_p2(&map));
}
