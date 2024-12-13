use std::{collections::HashMap, fs::read_to_string};

#[derive(Debug, PartialEq, Copy, Clone, Eq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

const MOVES: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

fn calculate_perimeter(v: &[Pos]) -> i32 {
    v.iter().fold(0, |acc, pos| {
        let mut p = 0;
        if !v.contains(&Pos {
            x: pos.x + 1,
            y: pos.y,
        }) {
            p += 1;
        }
        if !v.contains(&Pos {
            x: pos.x - 1,
            y: pos.y,
        }) {
            p += 1;
        }
        if !v.contains(&Pos {
            x: pos.x,
            y: pos.y + 1,
        }) {
            p += 1;
        }
        if !v.contains(&Pos {
            x: pos.x,
            y: pos.y - 1,
        }) {
            p += 1;
        }
        p + acc
    })
}

fn calculate_area(v: &[Pos]) -> i32 {
    v.len() as i32
}

fn find_neighbors(map: &HashMap<Pos, char>, pos: &Pos) -> Vec<Pos> {
    MOVES
        .iter()
        .map(|(dx, dy)| Pos {
            x: pos.x + dx,
            y: pos.y + dy,
        })
        .filter(|p| map.contains_key(p) && *map.get(p).unwrap() == *map.get(pos).unwrap())
        .collect()
}

fn dfs(map: &HashMap<Pos, char>, start: Pos, visited: &mut HashMap<Pos, char>) {
    visited.insert(start, *map.get(&start).unwrap());
    // println!("Start {:?}", map.get_key_value(&start).unwrap());

    for neighbor in find_neighbors(map, &start) {
        if !visited.contains_key(&neighbor) {
            dfs(map, neighbor, visited);
        }
    }
}

fn find_start(map: &HashMap<Pos, char>, visited: &HashMap<String, Vec<Pos>>) -> Option<Pos> {
    for (k, _) in map.iter() {
        if !visited.values().any(|v| v.contains(k)) {
            return Some(*k);
        }
    }
    None
}

fn solve_p1(map: &HashMap<Pos, char>) -> i32 {
    let mut garden: HashMap<String, Vec<Pos>> = HashMap::new();

    let mut i = 0;

    while let Some(s) = find_start(map, &garden) {
        let mut visited = HashMap::new();
        dfs(map, s, &mut visited);
        // println!("visited -> {:?}", visited);

        garden.insert(
            map[&s].to_string() + "-" + &i.to_string(),
            visited.keys().cloned().collect(),
        );
        // println!("garden -> {:?}", garden);
        i += 1;
    }
    let mut total = 0;
    garden.iter().for_each(|(k, v)| {
        let area = calculate_area(v);
        let perimeter = calculate_perimeter(v);
        // println!(
        //     "Plot {:?} -> Area: {:?}, Perimeter: {:?}",
        //     k, area, perimeter
        // );
        total += area * perimeter;
    });

    total
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    // println!("{:?}", input);

    let plots = input
        .lines()
        .enumerate()
        .flat_map(|(x, line)| {
            line.chars().enumerate().map(move |(y, c)| {
                (
                    Pos {
                        x: x as i32,
                        y: y as i32,
                    },
                    c,
                )
            })
        })
        .fold(HashMap::new(), |mut acc, (pos, c)| {
            acc.insert(pos, c);
            acc
        });

    // println!("{:?}", plots);
    println!("P1 -> {:?}", solve_p1(&plots));
}
