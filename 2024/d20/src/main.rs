use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Pos {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Grid {
    map: HashMap<Pos, char>,
    start: Pos,
    end: Pos,
    size: Pos,
}

const MOVES: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

impl Grid {
    fn find_neighbors(&self, pos: &Pos) -> Vec<Pos> {
        MOVES
            .iter()
            .map(|(dx, dy)| Pos {
                x: pos.x + dx,
                y: pos.y + dy,
            })
            .filter(|p| {
                p.x < self.size.x
                    && p.y < self.size.y
                    && p.x >= 0
                    && p.y >= 0
                    && self.map.get(p).unwrap() != &'#'
            })
            .collect()
    }

    fn bfs(&self, parent: &mut HashMap<Pos, Pos>, dist: &mut HashMap<Pos, i32>) -> i32 {
        let mut q = vec![self.start];
        dist.insert(self.start, 0);

        while !q.is_empty() {
            let pos = q.remove(0);
            // println!("checking {:?}", pos);
            if pos == self.end {
                return *dist.get(&pos).unwrap();
            }
            let neighbors = self.find_neighbors(&pos);
            // println!("neighbors: {:?}", neighbors);
            for neighbor in neighbors {
                if !dist.contains_key(&neighbor) {
                    dist.insert(neighbor, dist.get(&pos).unwrap() + 1);
                    parent.insert(neighbor, pos);
                    q.push(neighbor);
                }
            }
        }

        0
    }
}

fn solve_p1(grid: &Grid) {
    let mut parent = HashMap::new();
    let mut dist = HashMap::new();
    let distance = grid.bfs(&mut parent, &mut dist);
    println!("P1 -> {}", distance);
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    // println!("{}", input);

    let grid: Grid = input.lines().enumerate().fold(
        Grid {
            map: HashMap::new(),
            start: Pos { x: 0, y: 0 },
            end: Pos { x: 0, y: 0 },
            size: Pos { x: 0, y: 0 },
        },
        |mut grid, (y, line)| {
            grid.size.y = y as i32 + 1;
            line.chars().enumerate().for_each(|(x, c)| {
                let pos = Pos {
                    x: x as i32,
                    y: y as i32,
                };
                grid.size.x = x as i32 + 1;
                grid.map.insert(pos, c);
                if c == 'S' {
                    grid.start = pos;
                } else if c == 'E' {
                    grid.end = pos;
                }
            });
            grid
        },
    );

    // println!("{:?}", grid);
    solve_p1(&grid);
}
