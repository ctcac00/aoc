use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead};
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn write_output_file(result: &[u8]) -> std::io::Result<()> {
    let mut f = File::create("output.txt")?;
    f.write_all(result)?;
    Ok(())
}

fn check_safe(report: Vec<i32>) -> bool {
    println!("{:?}", report);
    let mut increasing = false;
    let mut decreasing = false;
    let mut is_safe = false;
    let mut i = 0;
    while i < report.len() - 1 {
        if report[i + 1] > report[i] && (report[i + 1] - report[i]) <= 3 {
            if decreasing {
                is_safe = false;
                break;
            } else {
                increasing = true;
                is_safe = true;
            }
        } else if (report[i + 1] < report[i]) && ((report[i] - report[i + 1]) <= 3) {
            if increasing {
                is_safe = false;
                break;
            } else {
                decreasing = true;
                is_safe = true;
            }
        } else {
            is_safe = false;
            break;
        }
        i += 1;
    }
    is_safe
}

fn solve(reports: Vec<Vec<i32>>) -> i32 {
    let mut safe = 0;
    for report in reports {
        let is_safe = check_safe(report);
        if is_safe {
            safe += 1;
        }
    }

    safe
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // check if the user has provided the input file
    if args.len() < 2 {
        eprintln!("Please provide the input file");
        std::process::exit(1);
    }

    let file_path = args[1].clone();
    if let Ok(lines) = read_lines(&file_path) {
        let mut reports: Vec<Vec<i32>> = Vec::new();

        // Consumes the iterator, returns an (Optional) String
        for line in lines.map_while(Result::ok) {
            let numbers: Vec<&str> = line.split_whitespace().collect();
            let mut report: Vec<i32> = Vec::new();
            for number in numbers {
                report.push(number.parse::<i32>().unwrap());
            }
            reports.push(report);
        }

        let result = solve(reports);
        println!("Result is {:?}", result);
        if let Err(e) = write_output_file(result.to_string().as_bytes()) {
            eprintln!("Error writing the output file: {}", e);
            std::process::exit(1);
        }
    } else {
        eprintln!("Error reading the file {}", file_path);
        std::process::exit(1);
    }
}
