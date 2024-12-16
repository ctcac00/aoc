use std::fs::read_to_string;

const BATHROOM: Pos = Pos { x: 101, y: 103 };

#[derive(Debug)]
struct Pos {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Robot {
    pos: Pos,
    vel: Pos,
}

impl Robot {
    fn step(&mut self) {
        self.pos.x += self.vel.x;
        self.pos.y += self.vel.y;

        if self.pos.x < 0 {
            self.pos.x += BATHROOM.x;
        } else if self.pos.x >= BATHROOM.x {
            self.pos.x -= BATHROOM.x;
        }
        if self.pos.y < 0 {
            self.pos.y += BATHROOM.y;
        } else if self.pos.y >= BATHROOM.y {
            self.pos.y -= BATHROOM.y;
        }
    }
}

fn print_map(robots: &[Robot]) {
    let mut map = vec![vec!['.'; BATHROOM.x as usize]; BATHROOM.y as usize];
    // count how many robots are in each position
    map.iter_mut().enumerate().for_each(|(y, row)| {
        row.iter_mut().enumerate().for_each(|(x, cell)| {
            let mut count = 0;
            for r in robots.iter() {
                if r.pos.x == x as i32 && r.pos.y == y as i32 {
                    count += 1;
                }
            }
            if count > 0 {
                *cell = (count as u8 + b'0') as char;
            }
            print!("{}", *cell);
        });
        println!();
    });
}

fn count_robots(robots: &[Robot]) -> usize {
    let mut quandrant_1 = 0;
    let mut quandrant_2 = 0;
    let mut quandrant_3 = 0;
    let mut quandrant_4 = 0;
    for r in robots.iter() {
        if r.pos.x < BATHROOM.x / 2 && r.pos.y < BATHROOM.y / 2 {
            quandrant_1 += 1;
        } else if r.pos.x < BATHROOM.x / 2 && r.pos.y > BATHROOM.y / 2 {
            quandrant_3 += 1;
        } else if r.pos.x > BATHROOM.x / 2 && r.pos.y < BATHROOM.y / 2 {
            quandrant_2 += 1;
        } else if r.pos.x > BATHROOM.x / 2 && r.pos.y > BATHROOM.y / 2 {
            quandrant_4 += 1;
        }
    }
    quandrant_1 * quandrant_2 * quandrant_3 * quandrant_4
}

fn solve_p1(robots: &mut [Robot]) {
    for seconds in 1..101 {
        for r in robots.iter_mut() {
            r.step();
        }
        // println!("After {:?} seconds:", seconds);
        // print_map(robots);
    }
    // println!("{:?}", robots);
    println!("P1 -> {:?} ", count_robots(robots));
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    // println!("{:?}", input);

    let mut robots = input
        .lines()
        .map(|l| {
            let re = regex::Regex::new(r"(p=(\d*),(\d*) v=(-?\d*),(-?\d*))").unwrap();
            let pos = Pos {
                x: re
                    .captures(l)
                    .unwrap()
                    .get(2)
                    .unwrap()
                    .as_str()
                    .parse::<i32>()
                    .unwrap(),
                y: re
                    .captures(l)
                    .unwrap()
                    .get(3)
                    .unwrap()
                    .as_str()
                    .parse::<i32>()
                    .unwrap(),
            };

            let vel = Pos {
                x: re
                    .captures(l)
                    .unwrap()
                    .get(4)
                    .unwrap()
                    .as_str()
                    .parse::<i32>()
                    .unwrap(),
                y: re
                    .captures(l)
                    .unwrap()
                    .get(5)
                    .unwrap()
                    .as_str()
                    .parse::<i32>()
                    .unwrap(),
            };

            Robot { pos, vel }
        })
        .collect::<Vec<_>>();
    // println!("{:?}", robots);
    // print_map(&robots);
    // println!();
    solve_p1(&mut robots);
}
