use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Pos {
    x: i32,
    y: i32,
}

fn find_antennas(map: &[Vec<char>]) -> HashMap<char, Vec<Pos>> {
    let mut antennas = HashMap::new();
    for (i, row) in map.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if *cell != '.' {
                antennas.entry(*cell).or_insert_with(Vec::new).push(Pos {
                    x: i as i32,
                    y: j as i32,
                });
            }
        }
    }
    antennas
}

fn find_antinodes_p2(map_size: Pos, antenna_a: Pos, antenna_b: Pos) -> Vec<Pos> {
    let mut antinodes = Vec::new();
    let mut antinode_1 = Pos { x: 0, y: 0 };
    let mut antinode_2 = Pos { x: 0, y: 0 };

    // println!("Antennas {:?} {:?}", antenna_a, antenna_b);

    let dx = antenna_b.x - antenna_a.x;
    let dy = antenna_b.y - antenna_a.y;

    let mut a_x = antenna_a.x;
    let mut a_y = antenna_a.y;
    let mut b_x = antenna_b.x;
    let mut b_y = antenna_b.y;

    // calculate antinodes
    (0..map_size.y).for_each(|_| {
        if dx >= 0 {
            antinode_1.x = a_x - dx.abs();
            antinode_2.x = b_x + dx.abs();

            a_x = antinode_1.x;
            b_x = antinode_2.x;
        } else {
            antinode_1.x = a_x + dx.abs();
            antinode_2.x = b_x - dx.abs();

            a_x = antinode_1.x;
            b_x = antinode_2.x;
        }

        if dy >= 0 {
            antinode_1.y = a_y - dy.abs();
            antinode_2.y = b_y + dy.abs();

            a_y = antinode_1.y;
            b_y = antinode_2.y;
        } else {
            antinode_1.y = a_y + dy.abs();
            antinode_2.y = b_y - dy.abs();

            a_y = antinode_1.y;
            b_y = antinode_2.y;
        }

        // println!(
        //     "New starting points {:?} {:?}",
        //     Pos { x: a_x, y: a_y },
        //     Pos { x: b_x, y: b_y }
        // );

        // if antinode_1 is in bounds
        if antinode_1.x >= 0
            && antinode_1.x < map_size.x
            && antinode_1.y >= 0
            && antinode_1.y < map_size.y
        {
            antinodes.push(antinode_1);
        }
        // if antinode_2 is in bounds
        if antinode_2.x >= 0
            && antinode_2.x < map_size.x
            && antinode_2.y >= 0
            && antinode_2.y < map_size.y
        {
            antinodes.push(antinode_2)
        }
    });

    // println!("Antinodes found {:?}", antinodes);
    antinodes
}

fn find_antinodes_p1(map_size: Pos, antenna_a: Pos, antenna_b: Pos) -> Vec<Pos> {
    let mut antinodes = Vec::new();
    let mut antinode_1 = Pos { x: 0, y: 0 };
    let mut antinode_2 = Pos { x: 0, y: 0 };

    // println!("Antennas {:?} {:?}", antenna_a, antenna_b);

    let dx = antenna_b.x - antenna_a.x;
    let dy = antenna_b.y - antenna_a.y;

    if dx >= 0 {
        antinode_1.x = antenna_a.x - dx.abs();
        antinode_2.x = antenna_b.x + dx.abs();
    } else {
        antinode_1.x = antenna_a.x + dx.abs();
        antinode_2.x = antenna_b.x - dx.abs();
    }

    if dy >= 0 {
        antinode_1.y = antenna_a.y - dy.abs();
        antinode_2.y = antenna_b.y + dy.abs();
    } else {
        antinode_1.y = antenna_a.y + dy.abs();
        antinode_2.y = antenna_b.y - dy.abs();
    }

    // if antinode_1 is in bounds
    if antinode_1.x >= 0
        && antinode_1.x < map_size.x
        && antinode_1.y >= 0
        && antinode_1.y < map_size.y
    {
        antinodes.push(antinode_1);
    }
    // if antinode_2 is in bounds
    if antinode_2.x >= 0
        && antinode_2.x < map_size.x
        && antinode_2.y >= 0
        && antinode_2.y < map_size.y
    {
        antinodes.push(antinode_2)
    }

    // println!("Antinodes found {:?}", antinodes);

    antinodes
}

fn solve_p2(map: &mut [Vec<char>]) -> i32 {
    //check if the antennas are in line
    let mut antennas = find_antennas(map);
    // println!("Antennas {:?}", antennas);

    // for each pair of antennas, find the antinodes
    let mut antinodes = Vec::new();
    for antenna in antennas.values() {
        // println!("Antenna {:?}", antenna);
        // for each pair of antennas
        for i in 0..antenna.len() {
            for j in i + 1..antenna.len() {
                let new_antinodes = find_antinodes_p2(
                    Pos {
                        x: map.len() as i32,
                        y: map[0].len() as i32,
                    },
                    antenna[i],
                    antenna[j],
                );

                // add anti nodes to map
                // for antinode in new_antinodes.iter() {
                // map[antinode.x as usize][antinode.y as usize] = '#';
                // }
                // print_map(map);

                antinodes.extend(new_antinodes);
            }
        }
    }
    let mut unique_locations = HashSet::new();
    for antinode in antinodes.iter() {
        if map[antinode.x as usize][antinode.y as usize] == '.' {
            unique_locations.insert(antinode);
        }
    }
    // println!("Antinodes {:?}", antinodes);
    // println!("Unique antinodes {:?}", unique_locations);

    // count all antenas that have at least 2 entries
    antennas.retain(|_, v| v.len() > 1);

    unique_locations.len() as i32 + antennas.values().flatten().collect::<Vec<&Pos>>().len() as i32
}

fn solve_p1(map: &mut [Vec<char>]) -> i32 {
    //check if the antennas are in line
    let antennas = find_antennas(map);
    // println!("Antennas {:?}", antennas);

    // for each pair of antennas, find the antinodes
    let mut antinodes = Vec::new();
    for antenna in antennas.values() {
        // println!("Antenna {:?}", antenna);
        // for each pair of antennas
        for i in 0..antenna.len() {
            for j in i + 1..antenna.len() {
                let new_antinodes = find_antinodes_p1(
                    Pos {
                        x: map.len() as i32,
                        y: map[0].len() as i32,
                    },
                    antenna[i],
                    antenna[j],
                );

                // add anti nodes to map
                // for antinode in new_antinodes.iter() {
                // map[antinode.x as usize][antinode.y as usize] = '#';
                // }
                // print_map(map);

                antinodes.extend(new_antinodes);
            }
        }
    }
    let mut unique_locations = HashSet::new();
    for antinode in antinodes.iter() {
        unique_locations.insert(antinode);
    }
    // println!("Antinodes {:?}", antinodes);
    // println!("Unique antinodes {:?}", unique_locations);

    unique_locations.len() as i32
}

fn print_map(map: &[Vec<char>]) {
    for row in map {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }
    println!();
}

fn main() {
    let input = read_to_string("input.txt").unwrap();

    let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    print_map(&map);
    println!("P1: {:?}", solve_p1(&mut map.clone()));
    println!("P2: {:?}", solve_p2(&mut map.clone()));
}
