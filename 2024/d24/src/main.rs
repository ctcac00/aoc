use std::{collections::HashMap, fs::read_to_string};

#[derive(Debug)]
struct Device {
    input_1: String,
    input_2: String,
    output: String,
    op: String,
    processed: bool,
}

fn solve_p1(devices: &mut [Device], conn: &mut HashMap<String, String>) {
    while devices.iter().any(|device| !device.processed) {
        for device in devices.iter_mut().filter(|device| !device.processed) {
            if !conn.contains_key(&device.input_1) || !conn.contains_key(&device.input_2) {
                continue;
            }

            let input_1 = conn.get(&device.input_1).unwrap();
            let input_2 = conn.get(&device.input_2).unwrap();

            match device.op.as_str() {
                "AND" => {
                    if input_1 == "1" && input_2 == "1" {
                        conn.insert(device.output.clone(), "1".to_string());
                    } else {
                        conn.insert(device.output.clone(), "0".to_string());
                    }
                }
                "OR" => {
                    if input_1 == "0" && input_2 == "0" {
                        conn.insert(device.output.clone(), "0".to_string());
                    } else {
                        conn.insert(device.output.clone(), "1".to_string());
                    }
                }
                "XOR" => {
                    if input_1 != input_2 {
                        conn.insert(device.output.clone(), "1".to_string());
                    } else {
                        conn.insert(device.output.clone(), "0".to_string());
                    }
                }
                _ => panic!("Unknown operation"),
            };

            device.processed = true;
        }
    }
    println!("{:?}", conn);
    println!("{:?}", devices);
    let mut result: String = String::new();
    let mut sorted_vec: Vec<_> = conn.iter_mut().collect();
    sorted_vec.sort_by(|a, b| b.0.cmp(a.0));
    sorted_vec.iter().for_each(|(key, value)| {
        if key.starts_with("z") {
            result += value;
        }
    });
    println!("{:?}", sorted_vec);
    println!("{}", result);
    println!("{}", u64::from_str_radix(&result, 2).unwrap());
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    println!("{}", input);
    let mut conn = HashMap::new();
    let mut devices = Vec::new();

    input.lines().for_each(|line| {
        if line.contains(":") {
            let mut parts = line.split(": ");
            let key = parts.next().unwrap();
            let value = parts.next().unwrap();
            conn.insert(key.to_string(), value.to_string());
        } else if line.contains(" -> ") {
            let mut parts = line.split_whitespace();
            let input_1 = parts.next().unwrap();
            let op = parts.next().unwrap();
            let input_2 = parts.next().unwrap();
            let output = parts.last().unwrap();
            devices.push(Device {
                input_1: input_1.to_string(),
                input_2: input_2.to_string(),
                output: output.to_string(),
                op: op.to_string(),
                processed: false,
            });
        }
    });

    println!("{:?}", conn);
    println!("{:?}", devices);
    solve_p1(&mut devices, &mut conn);
}
