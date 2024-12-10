use std::fs::read_to_string;

#[derive(Debug)]
struct File {
    id: usize,
    used_space: i32,
    free_space: i32,
}

fn print_filesystem(files: &[File]) {
    for file in files {
        print!(
            "{:?}",
            file.id.to_string().repeat(file.used_space as usize)
                + &".".repeat(file.free_space as usize)
        );
    }
    println!();
}

fn find_file_with_empty_space(files: &[File]) -> usize {
    files.iter().position(|file| file.free_space > 0).unwrap()
}

fn find_last_file_with_used_space(files: &[File]) -> usize {
    files.iter().rposition(|file| file.used_space > 0).unwrap()
}

fn move_file_block(files: &mut Vec<File>, file_a: usize, file_b: usize) {
    files[file_b].used_space -= 1;
    files[file_b].free_space += 1;

    files.insert(
        file_a + 1,
        File {
            id: files[file_b].id,
            used_space: 1,
            free_space: files[file_a].free_space - 1,
        },
    );

    files[file_a].free_space = 0;
}

fn checksum(files: &[File]) -> u64 {
    let mut index = 0;
    let mut total = 0;
    for file in files {
        (0..file.used_space).for_each(|_| {
            total += index as u64 * file.id as u64;
            index += 1;
        });
    }

    total
}

fn solve_p1(files: &mut Vec<File>) -> u64 {
    print_filesystem(files);

    loop {
        let file_a = find_file_with_empty_space(files);
        let file_b = find_last_file_with_used_space(files);

        if file_a >= file_b {
            break;
        }

        move_file_block(files, file_a, file_b);

        // println!("{:?}", files);
        // print_filesystem(files);
    }

    println!("{:?}", files);
    print_filesystem(files);

    checksum(files)
}

fn main() {
    let input = read_to_string("input.txt").unwrap().replace("\n", "");
    println!("{:?}", input);

    let mut files = input
        .chars()
        .collect::<Vec<_>>()
        .chunks(2)
        .enumerate()
        .map(|(index, w)| {
            if w.len() == 1 {
                File {
                    id: index,
                    used_space: w[0].to_digit(10).unwrap_or(0) as i32,
                    free_space: 0,
                }
            } else {
                File {
                    id: index,
                    used_space: w[0].to_digit(10).unwrap_or(0) as i32,
                    free_space: w[1].to_digit(10).unwrap_or(0) as i32,
                }
            }
        })
        .collect::<Vec<_>>();
    println!("{:?}", files);
    println!("{:?}", solve_p1(&mut files));
}
