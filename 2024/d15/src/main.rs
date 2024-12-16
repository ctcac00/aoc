use std::{collections::HashMap, fs::read_to_string};

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Pos {
    x: i32,
    y: i32,
}

fn is_move_valid(map: &HashMap<Pos, char>, pos: &Pos, direction: char) -> bool {
    match map.get(pos) {
        Some(c) => {
            if *c == '#' {
                return false;
            } else if *c == '.' {
                return true;
            } else if *c == 'O' {
                return is_move_valid(map, &get_move(pos, direction), direction);
            }
        }
        None => {
            return false;
        }
    }

    true
}

fn get_move(pos: &Pos, direction: char) -> Pos {
    match direction {
        '^' => Pos {
            x: pos.x - 1,
            y: pos.y,
        },
        'v' => Pos {
            x: pos.x + 1,
            y: pos.y,
        },
        '<' => Pos {
            x: pos.x,
            y: pos.y - 1,
        },
        '>' => Pos {
            x: pos.x,
            y: pos.y + 1,
        },
        _ => Pos { x: pos.x, y: pos.y },
    }
}

fn find_free_space(map: &HashMap<Pos, char>, pos: Pos, direction: char) -> Pos {
    let mut free_space = Pos { x: 0, y: 0 };
    let mut next_pos = get_move(&pos, direction);
    loop {
        if map.get(&next_pos).unwrap() == &'.' {
            free_space.x = next_pos.x;
            free_space.y = next_pos.y;
            break;
        }

        next_pos = get_move(&next_pos, direction);
    }

    free_space
}

fn move_box(map: &mut HashMap<Pos, char>, next_pos: Pos, direction: char, stop: Pos) {
    let mut next_pos = next_pos;
    while next_pos != stop {
        next_pos = get_move(&next_pos, direction);
        map.entry(next_pos).and_modify(|v| *v = 'O');
    }
    // print_map(map);
}

fn solve_p1(map: &mut HashMap<Pos, char>, robot: &mut Pos, moves: &[char]) -> i32 {
    (0..moves.len()).for_each(|i| {
        let direction = moves[i];
        // println!("Move {}:", direction);
        let next_pos = get_move(robot, direction);
        if is_move_valid(map, &next_pos, direction) {
            if map.get(&next_pos).unwrap() == &'.' {
                map.entry(next_pos).and_modify(|v| *v = '@');
                map.entry(*robot).and_modify(|v| *v = '.');
                robot.x = next_pos.x;
                robot.y = next_pos.y;
            } else {
                let free_space = find_free_space(map, next_pos, direction);
                move_box(map, next_pos, direction, free_space);

                map.entry(next_pos).and_modify(|v| *v = '@');
                map.entry(*robot).and_modify(|v| *v = '.');
                robot.x = next_pos.x;
                robot.y = next_pos.y;
            }
        }
        // print_map(map);
    });

    map.iter().fold(0, |acc, (k, v)| {
        if v == &'O' {
            acc + (k.x * 100 + k.y)
        } else {
            acc
        }
    })
}

fn print_map(map: &HashMap<Pos, char>) {
    for x in 0..8 {
        for y in 0..8 {
            match map.get(&Pos { x, y }) {
                Some(c) => print!("{}", c),
                None => print!(" "),
            }
        }
        println!();
    }
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    // println!("{:?}", input);

    let mut map: HashMap<Pos, char> = HashMap::new();
    let mut moves: Vec<char> = Vec::new();
    let mut robot: Pos = Pos { x: 0, y: 0 };
    input.lines().enumerate().for_each(|(x, line)| {
        if line.contains("#") {
            line.chars().enumerate().for_each(|(y, c)| {
                if c == '@' {
                    robot.x = x as i32;
                    robot.y = y as i32;
                }

                map.insert(
                    Pos {
                        x: x as i32,
                        y: y as i32,
                    },
                    c,
                );
            });
        } else if line.len() > 1 {
            line.chars().for_each(|c| moves.push(c));
        }
    });

    // println!("{:?}", map);
    // println!("{:?}", robot);
    // println!("{:?}", moves);
    println!("P1 -> {:?}", solve_p1(&mut map, &mut robot, &moves));
}
