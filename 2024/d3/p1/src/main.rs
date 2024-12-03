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

fn main() {
    let args: Vec<String> = env::args().collect();
    // check if the user has provided the input file
    if args.len() < 2 {
        eprintln!("Please provide the input file");
        std::process::exit(1);
    }

    let file_path = args[1].clone();
    let mut result = 0;
    if let Ok(lines) = read_lines(&file_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.map_while(Result::ok) {
            println!("{}", line);
            let re = regex::Regex::new(r"mul\(\d*,\d*\)").unwrap();

            let it = re.captures_iter(&line);
            for caps in it {
                println!("Captures: {:?}", caps);
                let mut calc = caps[0].to_string().split_off(4);
                calc.pop();
                let numbers: Vec<&str> = calc.split(",").collect();
                let left = numbers[0].parse::<i32>().unwrap();
                let right = numbers[1].parse::<i32>().unwrap();
                result += left * right;
            }
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
