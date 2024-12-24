use std::fs::read_to_string;

fn prune(secret: u64) -> u64 {
    secret % 16777216
}

fn mix(secret: u64, value: u64) -> u64 {
    secret ^ value
}

fn process_secret(secret: &u64) -> u64 {
    let mut secret = *secret;

    // multiply by 64
    let mut value = secret * 64;

    // mix
    secret = mix(secret, value);
    // prune
    secret = prune(secret);

    // divide by 32
    value = (secret as f64 / 32.0).floor() as u64;

    // mix
    secret = mix(secret, value);
    // prune
    secret = prune(secret);

    // multiply by 2048
    value = secret * 2048;

    // mix
    secret = mix(secret, value);
    // prune
    secret = prune(secret);

    secret
}

fn solve_p1(secrets: &[u64]) {
    let mut results = Vec::new();
    for secret in secrets {
        let mut secret = *secret;
        for _ in 0..2000 {
            secret = process_secret(&secret);
        }
        results.push(secret);
    }
    println!("P1-> {:?}", results.iter().sum::<u64>());
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    // println!("{}", input);

    let secrets = input
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    // println!("{:?}", secrets);
    solve_p1(&secrets);
}
