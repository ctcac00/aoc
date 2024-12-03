use std::fs::File;
use std::io::{BufRead, Write};
use std::path::Path;
use std::{env, io};

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

fn solve(original_report: Vec<i32>, index: i32) -> i32 {
    let mut i = 0;
    let mut safe = false;

    let mut report = original_report.clone();
    if index >= 0 {
        report.remove(index as usize);
    }

    while i < report.len() - 1 {
        if i != 0 && i != report.len() - 1 {
            // compare difference with the previous element
            let diff_previous = report[i] - report[i - 1];
            //compare difference with the next element
            let diff_next = report[i] - report[i + 1];

            if diff_previous > 0
                && diff_next < 0
                && (1..=3).contains(&diff_previous)
                && (-3..=-1).contains(&diff_next)
            {
                // bigger than the previous but smaller than the next
                // increasing
                safe = true;
            } else if diff_previous < 0
                && diff_next > 0
                && (-3..=-1).contains(&diff_previous)
                && (1..=3).contains(&diff_next)
            {
                // smaller than the previous but bigger than the next
                // decreasing
                safe = true;
            } else {
                safe = false;
                break;
            }
        }
        i += 1;
    }

    if safe {
        1
    } else if index < report.len() as i32 {
        return solve(original_report, index + 1);
    } else {
        0
    }
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
        // Consumes the iterator, returns an (Optional) String
        let mut result = 0;
        for line in lines.map_while(Result::ok) {
            let report = line
                .split_whitespace()
                .map(|s| s.parse::<i32>())
                .collect::<Result<Vec<_>, _>>()
                .unwrap();

            println!("{:?}", report);
            let solution = solve(report, -1);
            if solution == 1 {
                println!("Safe");
            }
            result += solution;
        }

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
