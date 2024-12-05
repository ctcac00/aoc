use std::{collections::HashMap, fs::read_to_string};

fn solve(rules: HashMap<i32, Vec<i32>>, pages: Vec<Vec<i32>>) -> i32 {
    let mut result = 0;

    for page in pages {
        let mut in_order = true;
        for num in page.iter().rev() {
            let page_pos = page.iter().position(|&r| r == *num).unwrap();

            let check = rules.get(num);
            if check.is_some() {
                for x in check.unwrap() {
                    if page.contains(x) {
                        if let Some(pos) = page.iter().position(|&r| r == *x) {
                            if page_pos < pos {
                                in_order = true;
                            } else {
                                println!("{:?} broke the rule {:?} in page {:?}", num, check, page);
                                in_order = false;
                                break;
                            }
                        }
                    }
                }
            }

            if !in_order {
                break;
            }
        }
        if in_order {
            println!("Page {:?} is in order", page);
            let middle_page = page.get(page.len() / 2).unwrap_or(&0);
            result += middle_page;
        }
    }
    result
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let mut rules = HashMap::new();
    let mut pages: Vec<Vec<i32>> = Vec::new();
    for line in input.lines() {
        if line.contains("|") {
            let (left, right): (i32, i32) = {
                let numbers: Vec<i32> = line.split('|').map(|s| s.parse().unwrap()).collect();
                (numbers[0], numbers[1])
            };
            rules.entry(left).or_insert_with(Vec::new).push(right);
        } else if line.contains(",") {
            pages.push(
                line.split(',')
                    .collect::<Vec<_>>()
                    .iter()
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect(),
            );
        }
    }

    println!("{:}", solve(rules, pages));
}
