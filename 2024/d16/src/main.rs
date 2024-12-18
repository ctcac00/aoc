use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
    fs::read_to_string,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Pos {
    x: i32,
    y: i32,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
struct Reindeer {
    pos: Pos,
    direction: Direction,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    score: usize,
    position: Pos,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Move {
    pos: Pos,
    direction: Direction,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that we flip the ordering on scores.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .score
            .cmp(&self.score)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
fn print_map(map: &HashMap<Pos, char>, map_size: Pos) {
    for x in 0..(map_size.x + 1) {
        for y in 0..(map_size.y + 1) {
            match map.get(&Pos { x, y }) {
                Some(c) => print!("{}", c),
                None => print!(" "),
            }
        }
        println!();
    }
}

fn find_neighbours(map: &HashMap<Pos, char>, pos: &Pos) -> Vec<Move> {
    let neighbours: Vec<Move> = vec![
        Move {
            pos: Pos {
                x: pos.x - 1,
                y: pos.y,
            },
            direction: Direction::North,
        },
        Move {
            pos: Pos {
                x: pos.x + 1,
                y: pos.y,
            },
            direction: Direction::South,
        },
        Move {
            pos: Pos {
                x: pos.x,
                y: pos.y - 1,
            },
            direction: Direction::West,
        },
        Move {
            pos: Pos {
                x: pos.x,
                y: pos.y + 1,
            },
            direction: Direction::East,
        },
    ];

    neighbours
        .iter()
        .filter(|map_move| {
            map.get(&map_move.pos).is_some() && map.get(&map_move.pos).unwrap() != &'#'
        })
        .cloned() // Clone each `Move` instance
        .collect() // Collect into a Vec<Move>
}

fn dijkstra(
    map: &mut HashMap<Pos, char>,
    map_size: &Pos,
    reindeer: &mut Reindeer,
    end: &Pos,
) -> usize {
    let mut visited = HashSet::new();
    let mut dist = HashMap::new();
    let mut queue = BinaryHeap::new();
    let mut parents: HashMap<Pos, Move> = HashMap::new();

    queue.push(State {
        score: 0,
        position: reindeer.pos,
    });
    dist.insert(reindeer.pos, 0);
    parents.insert(
        reindeer.pos,
        Move {
            pos: reindeer.pos,
            direction: reindeer.direction,
        },
    );

    while !queue.is_empty() {
        let u = queue.pop().unwrap().position;

        if visited.contains(&u) {
            continue;
        }
        visited.insert(u);

        if u.x == end.x && u.y == end.y {
            // println!("Parents -> {:?}", parents);
            return *dist.get(&u).unwrap();
        }

        for neighbour in find_neighbours(map, &u) {
            if !visited.contains(&neighbour.pos) {
                let mut alt = dist.get(&u).unwrap_or(&0) + 1;
                let parent = parents.get(&u).unwrap();
                if parent.direction != neighbour.direction {
                    alt += 1000;
                }
                if alt < *dist.get(&neighbour.pos).unwrap_or(&1000000) {
                    dist.insert(neighbour.pos, alt);
                    queue.push(State {
                        score: alt,
                        position: neighbour.pos,
                    });
                    parents.insert(
                        neighbour.pos,
                        Move {
                            pos: u,
                            direction: neighbour.direction,
                        },
                    );
                    // println!("Reindeer -> {:?}", neighbour);
                    // print_map(map, *map_size);
                }
            }
        }
    }
    0
}

fn solve_p1(
    map: &mut HashMap<Pos, char>,
    map_size: &Pos,
    reindeer: &mut Reindeer,
    end: &Pos,
) -> usize {
    dijkstra(map, map_size, reindeer, end)
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    // println!("{}", input);

    let mut map: HashMap<Pos, char> = HashMap::new();
    let mut reindeer: Reindeer = Reindeer {
        pos: Pos { x: 0, y: 0 },
        direction: Direction::East,
    };
    let mut end: Pos = Pos { x: 0, y: 0 };
    let mut map_size = Pos { x: 0, y: 0 };

    input.lines().enumerate().for_each(|(x, line)| {
        line.chars().enumerate().for_each(|(y, c)| {
            if c == 'S' {
                reindeer.pos.x = x as i32;
                reindeer.pos.y = y as i32;
            } else if c == 'E' {
                end.x = x as i32;
                end.y = y as i32;
            }

            map.insert(
                Pos {
                    x: x as i32,
                    y: y as i32,
                },
                c,
            );

            map_size.x = x as i32;
            map_size.y = y as i32;
        });
    });

    // println!("{:?}", map);
    // println!("{:?}", reindeer);
    // println!("{:?}", end);
    // println!("{:?}", map_size);
    // print_map(&map, map_size);
    println!(
        "P1 -> {:?}",
        solve_p1(&mut map, &map_size, &mut reindeer, &end)
    );
}
