use std::{collections::HashMap, fs::read_to_string};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Pos {
    x: i32,
    y: i32,
}

const MOVES: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
const GRID_SIZE: Pos = Pos { x: 71, y: 71 };
const TEST_BYTES: usize = 1024;

fn print_grid(bytes: &[Pos]) {
    (0..GRID_SIZE.y).for_each(|y| {
        (0..GRID_SIZE.x).for_each(|x| {
            if bytes.iter().any(|pos| pos.x == x && pos.y == y) {
                print!("#");
            } else {
                print!(".");
            }
        });
        println!();
    });
}

fn find_neighbors(map: &[Pos], pos: &Pos) -> Vec<Pos> {
    MOVES
        .iter()
        .map(|(dx, dy)| Pos {
            x: pos.x + dx,
            y: pos.y + dy,
        })
        .filter(|p| {
            p.x < GRID_SIZE.x
                && p.y < GRID_SIZE.y
                && p.x >= 0
                && p.y >= 0
                && !map.iter().any(|pos| pos.x == p.x && pos.y == p.y)
        })
        .collect()
}

fn bfs(
    map: &[Pos],
    s: Pos,
    target: Pos,
    parent: &mut HashMap<Pos, Pos>,
    dist: &mut HashMap<Pos, i32>,
) -> bool {
    let mut q = vec![s];
    dist.insert(s, 0);

    while !q.is_empty() {
        let pos = q.remove(0);
        if pos == target {
            return true;
        }
        let neighbors = find_neighbors(map, &pos);
        for neighbor in neighbors {
            if !dist.contains_key(&neighbor) {
                dist.insert(neighbor, dist.get(&pos).unwrap() + 1);
                parent.insert(neighbor, pos);
                q.push(neighbor);
            }
        }
    }

    false
}

fn solve_p1(map: &[Pos]) {
    let s = Pos { x: 0, y: 0 };
    let target = Pos {
        x: GRID_SIZE.x - 1,
        y: GRID_SIZE.y - 1,
    };

    let mut parent = HashMap::new();
    let mut dist = HashMap::new();

    bfs(map, s, target, &mut parent, &mut dist);
    println!("P1 -> {:?}", dist.get(&target).unwrap());
}

fn solve_p2(map: &[Pos]) {
    let s = Pos { x: 0, y: 0 };
    let target = Pos {
        x: GRID_SIZE.x - 1,
        y: GRID_SIZE.y - 1,
    };

    let mut i = 0;

    loop {
        let mut parent = HashMap::new();
        let mut dist = HashMap::new();
        if !bfs(&map[..i], s, target, &mut parent, &mut dist) {
            println!("P2 -> {:?}", map.get(i - 1).unwrap());
            break;
        }
        i += 1;
        print!("{},", i);
    }
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    println!("{}", input);
    let bytes = input
        .lines()
        .map(|line| Pos {
            x: line.split(",").next().unwrap().parse::<i32>().unwrap(),
            y: line.split(",").last().unwrap().parse::<i32>().unwrap(),
        })
        .collect::<Vec<_>>();
    println!("{:?}", bytes);
    print_grid(&bytes[..TEST_BYTES]);
    solve_p1(&bytes[..TEST_BYTES]);
    solve_p2(&bytes);
}
