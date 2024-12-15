use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
    fs::read_to_string,
};

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: Pos,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Claw {
    prize: Pos,
    a: Pos,
    b: Pos,
}

impl Claw {
    fn dijkstra(&self) -> usize {
        let mut visited = HashSet::new();
        let mut dist = HashMap::new();
        let mut queue = BinaryHeap::new();

        queue.push(State {
            cost: 0,
            position: Pos { x: 0, y: 0 },
        });
        dist.insert(Pos { x: 0, y: 0 }, 0);

        while !queue.is_empty() {
            let u = queue.pop().unwrap().position;

            if visited.contains(&u) {
                continue;
            }
            visited.insert(u);

            if u.x == self.prize.x && u.y == self.prize.y {
                return *dist.get(&u).unwrap();
            } else if u.x > self.prize.x || u.y > self.prize.y {
                continue;
            }

            // process the neighbors
            let x = u.x + self.a.x;
            let y = u.y + self.a.y;
            if !visited.contains(&Pos { x, y }) {
                let alt = dist.get(&u).unwrap_or(&0) + 3;
                if alt < *dist.get(&Pos { x, y }).unwrap_or(&1000000) {
                    dist.insert(Pos { x, y }, alt);
                    queue.push(State {
                        cost: alt,
                        position: Pos { x, y },
                    });
                }
            }

            let x = u.x + self.b.x;
            let y = u.y + self.b.y;
            if !visited.contains(&Pos { x, y }) {
                let alt = dist.get(&u).unwrap_or(&0) + 1;
                if alt < *dist.get(&Pos { x, y }).unwrap_or(&1000000) {
                    dist.insert(Pos { x, y }, alt);
                    queue.push(State {
                        cost: alt,
                        position: Pos { x, y },
                    });
                }
            }
        }
        0
    }
}

fn solve_p1(claws: Vec<Claw>) -> usize {
    claws.iter().fold(0, |acc, claw| acc + claw.dijkstra())
}

fn main() {
    let input = read_to_string("input.txt").unwrap();

    // TODO: look for a different way to parse the input
    let claws = input
        .lines()
        .collect::<Vec<_>>()
        .chunks(4)
        .map(|w| {
            let re = regex::Regex::new(r"(Button A: X\+(\d*), Y\+(\d*))").unwrap();
            let a = Pos {
                x: re
                    .captures(w[0])
                    .unwrap()
                    .get(2)
                    .unwrap()
                    .as_str()
                    .parse::<usize>()
                    .unwrap(),
                y: re
                    .captures(w[0])
                    .unwrap()
                    .get(3)
                    .unwrap()
                    .as_str()
                    .parse::<usize>()
                    .unwrap(),
            };

            let re = regex::Regex::new(r"(Button B: X\+(\d*), Y\+(\d*))").unwrap();
            let b = Pos {
                x: re
                    .captures(w[1])
                    .unwrap()
                    .get(2)
                    .unwrap()
                    .as_str()
                    .parse::<usize>()
                    .unwrap(),
                y: re
                    .captures(w[1])
                    .unwrap()
                    .get(3)
                    .unwrap()
                    .as_str()
                    .parse::<usize>()
                    .unwrap(),
            };

            let re = regex::Regex::new(r"Prize: X=(\d*), Y=(\d*)").unwrap();

            let prize = Pos {
                x: re
                    .captures(w[2])
                    .unwrap()
                    .get(1)
                    .unwrap()
                    .as_str()
                    .parse::<usize>()
                    .unwrap(),
                y: re
                    .captures(w[2])
                    .unwrap()
                    .get(2)
                    .unwrap()
                    .as_str()
                    .parse::<usize>()
                    .unwrap(),
            };

            Claw { prize, a, b }
        })
        .collect::<Vec<_>>();

    println!("P1 -> {:?}", solve_p1(claws));
}
