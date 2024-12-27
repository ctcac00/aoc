use std::{collections::HashSet, fs::read_to_string};

fn bfs(towel: &str, stripes: &[&str]) -> bool {
    let mut q = vec![0];
    let mut visited = HashSet::new();
    visited.insert(0);

    while !q.is_empty() {
        let start = q.remove(0);
        if start == towel.len() {
            return true;
        }
        for neighbor in stripes {
            let end = start + neighbor.len();
            if !visited.contains(&end) && end <= towel.len() && &towel[start..end] == *neighbor {
                q.push(end);
                visited.insert(end);
            }
        }
    }

    false
}
fn solve_p1(towels: &[&str], stripes: &[&str]) {
    print!(
        "P1 -> {}",
        towels.iter().filter(|towel| bfs(towel, stripes)).count()
    );
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    // println!("{}", input);
    let mut towels = Vec::new();
    let mut stripes = Vec::new();
    input.lines().enumerate().for_each(|(i, line)| {
        if i == 0 {
            line.split(", ").for_each(|stripe| {
                stripes.push(stripe);
            });
        } else if i > 1 {
            towels.push(line);
        }
    });

    // println!("{:?}", stripes);
    // println!("{:?}", towels);
    solve_p1(&towels, &stripes);
}
