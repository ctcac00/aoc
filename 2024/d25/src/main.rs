fn overlap(key: &[i32], lock: &[i32]) -> bool {
    // check if adding each element of key to lock is greater than 6
    lock.iter().zip(key.iter()).any(|(l, k)| l + k > 6)
}

fn solve_p1(locks: Vec<Vec<i32>>, keys: Vec<Vec<i32>>) {
    let mut unique_pairs = std::collections::HashSet::new();

    locks.iter().for_each(|lock| {
        keys.iter().for_each(|key| {
            // print!("Testing Lock: {:?} with Key: {:?}", lock, key);
            if overlap(key, lock) {
                // print!(" key overlaps lock");
            } else {
                unique_pairs.insert((key, lock));
            }
            // println!();
        });
    });

    println!("P1 -> {:?}", unique_pairs.len());
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    // println!("{}", input);
    let mut keys: Vec<Vec<i32>> = Vec::new();
    let mut locks: Vec<Vec<i32>> = Vec::new();

    input
        .lines()
        .collect::<Vec<_>>()
        .chunks(8)
        .for_each(|chunk| {
            if chunk[0] == "#####" {
                // it's a lock
                let lock = chunk
                    .iter()
                    .map(|line| {
                        line.chars()
                            .map(|c| match c {
                                '#' => 1,
                                '.' => 0,
                                _ => panic!("Invalid char"),
                            })
                            .collect::<Vec<_>>()
                    })
                    .filter(|line| !line.is_empty())
                    .fold(vec![0; 5], |mut acc, row| {
                        row.iter().enumerate().for_each(|(i, &v)| {
                            acc[i] += v;
                        });
                        acc
                    });
                locks.push(lock);
            } else if chunk[0] == "....." {
                // it's a key
                let key = chunk
                    .iter()
                    .enumerate()
                    .map(|(i, line)| {
                        if i != 6 {
                            line.chars()
                                .map(|c| match c {
                                    '#' => 1,
                                    '.' => 0,
                                    _ => panic!("Invalid char"),
                                })
                                .collect::<Vec<_>>()
                        } else {
                            vec![0; 5]
                        }
                    })
                    .filter(|line| !line.is_empty())
                    .fold(vec![0; 5], |mut acc, row| {
                        row.iter().enumerate().for_each(|(i, &v)| {
                            acc[i] += v;
                        });
                        acc
                    });
                keys.push(key);
            }
        });

    // println!("{:?}", keys);
    // println!("Locks:");
    // println!("{:?}", locks);

    solve_p1(locks, keys);
}
