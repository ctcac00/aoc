use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

fn find_antennas(map: &[Vec<char>]) -> HashMap<char, Vec<(usize, usize)>> {
    let mut antennas = HashMap::new();
    for (i, row) in map.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if *cell != '.' {
                antennas.entry(*cell).or_insert_with(Vec::new).push((i, j));
            }
        }
    }
    antennas
}

fn find_antinodes(
    map_size: (usize, usize),
    antenna_a: (usize, usize),
    antenna_b: (usize, usize),
) -> Vec<(i32, i32)> {
    let mut antinodes = Vec::new();
    let mut antinode_1: (i32, i32) = (0, 0);
    let mut antinode_2: (i32, i32) = (0, 0);

    println!("Antennas {:?} {:?}", antenna_a, antenna_b);

    //convert to i32 to avoid negative values
    let antenna_a = (antenna_a.0 as i32, antenna_a.1 as i32);
    let antenna_b = (antenna_b.0 as i32, antenna_b.1 as i32);

    // TODO: calculate the antinodes points

    // if antinode_1 is in bounds
    if antinode_1.0 >= 0
        && antinode_1.0 < map_size.0 as i32
        && antinode_1.1 >= 0
        && antinode_1.1 < map_size.1 as i32
    {
        antinodes.push(antinode_1);
    }
    // if antinode_2 is in bounds
    if antinode_2.0 >= 0
        && antinode_2.0 < map_size.0 as i32
        && antinode_2.1 >= 0
        && antinode_2.1 < map_size.1 as i32
    {
        antinodes.push(antinode_2)
    }

    println!("Antinodes found {:?}", antinodes);

    antinodes
}

fn solve_p1(map: &mut [Vec<char>]) -> i32 {
    //check if the antennas are in line
    let antennas = find_antennas(map);
    println!("Antennas {:?}", antennas);

    // for each pair of antennas, find the antinodes
    let mut antinodes = Vec::new();
    for antenna in antennas.values() {
        println!("Antenna {:?}", antenna);
        // for each pair of antennas
        for i in 0..antenna.len() {
            for j in i + 1..antenna.len() {
                let new_antinodes =
                    find_antinodes((map.len(), map[0].len()), antenna[i], antenna[j]);

                // add anti nodes to map
                for antinode in new_antinodes.iter() {
                    map[antinode.0 as usize][antinode.1 as usize] = '#';
                }
                print_map(map);

                antinodes.extend(new_antinodes);
            }
        }
    }
    let mut unique_locations = HashSet::new();
    for antinode in antinodes.iter() {
        unique_locations.insert(antinode);
    }
    println!("Map size {:?}", (map.len(), map[0].len()));
    println!("Antinodes {:?}", antinodes);
    println!("Unique antinodes {:?}", unique_locations);

    unique_locations.len() as i32
}

fn print_map(map: &mut [Vec<char>]) {
    for row in map {
        println!("{:?}", row);
    }
    println!();
}

fn main() {
    let input = read_to_string("input.txt").unwrap();

    let mut map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    // print_map(&mut map);
    println!("{:?}", solve_p1(&mut map));
}
