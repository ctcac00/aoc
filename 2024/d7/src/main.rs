use std::fs::read_to_string;

fn apply_operation(op: &str, a: u64, b: u64) -> u64 {
    match op {
        "+" => a + b,
        "*" => a * b,
        _ => panic!("Invalid operation"),
    }
}

fn calculate(values: &[u64], operations: &[&str]) -> u64 {
    let mut total = 0;
    for i in 0..operations.len() {
        let mut left_value = total;
        if i == 0 {
            left_value = values[i];
        }

        // println!(
        //     "Applying {:?} to {:?} and {:?}",
        //     operations[i],
        //     left_value,
        //     values[i + 1]
        // );
        total = apply_operation(operations[i], left_value, values[i + 1]);
        // println!("{:?}", total);
    }

    total
}

fn is_solvable(total: u64, values: &[u64]) -> bool {
    // using all the values in the list and only the add and multiply operations
    // can we reach the total?
    let num_operations = values.len() - 1;
    let mut operations = vec![""; num_operations];
    (0..num_operations).for_each(|i| {
        operations[i] = "+";
    });

    // generate all possible combinations of operations
    for i in 0..2_usize.pow(num_operations as u32) {
        let mut n = i;
        (0..num_operations).for_each(|j| {
            if n % 2 == 0 {
                operations[j] = "+";
            } else {
                operations[j] = "*";
            }
            n /= 2;
        });
        // println!("{:?}", operations);

        if calculate(values, &operations) == total {
            return true;
        }
    }

    false
}

fn solve_p1(calibration: Vec<(u64, Vec<u64>)>) -> u64 {
    calibration
        .iter()
        .filter(|(total, values)| is_solvable(*total, values))
        .fold(0, |acc, calibration| acc + calibration.0)
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    // println!("{:?}", input);

    let calibration = input
        .lines()
        .map(|line| line.split(": ").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>()
        .iter()
        .map(|v| {
            (
                v[0].parse::<u64>().unwrap(),
                v[1].split_whitespace()
                    .collect::<Vec<_>>()
                    .iter()
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    // println!("{:?}", calibration);
    println!("{:?}", solve_p1(calibration));
}
