use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

fn intersect<T: std::cmp::Eq + std::hash::Hash + Clone>(arr1: &[T], arr2: &[T]) -> Vec<T> {
    let set1: HashSet<_> = arr1.iter().cloned().collect();
    let set2: HashSet<_> = arr2.iter().cloned().collect();
    set1.intersection(&set2).cloned().collect()
}

fn normalize_tuple(tuple: (&str, &str, &str)) -> (String, String, String) {
    let mut elements = [tuple.0, tuple.1, tuple.2];
    elements.sort();
    (
        elements[0].to_string().clone(),
        elements[1].to_string().clone(),
        elements[2].to_string().clone(),
    )
}

fn solve_p1(networks: &HashMap<&str, Vec<&str>>) {
    let mut connections = Vec::new();
    networks.iter().for_each(|(k, v)| {
        v.iter().for_each(|n| {
            let conn = networks.get(n).unwrap();
            let intersection = intersect(v, conn);
            if !intersection.is_empty() {
                intersection.iter().for_each(|i| {
                    connections.push((*k, *n, *i));
                });
            }
        });
    });

    let unique_tuples: HashSet<(String, String, String)> = connections
        .iter()
        .map(|tuple| normalize_tuple(*tuple))
        .collect();

    println!(
        "P1 -> {:?}",
        unique_tuples
            .iter()
            .filter(|tuple| tuple.0.starts_with('t')
                || tuple.1.starts_with('t')
                || tuple.2.starts_with('t'))
            .count()
    );
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let mut networks: HashMap<&str, Vec<&str>> = HashMap::new();

    input.lines().for_each(|line| {
        let mut parts = line.split('-');
        let a = parts.next().unwrap();
        let b = parts.next().unwrap();
        networks.entry(a).or_default().push(b);
        networks.entry(b).or_default().push(a);
    });

    // println!("{:?}", networks);
    solve_p1(&networks);
}
