use std::fs::read_to_string;

#[derive(Debug)]
struct Computer {
    register_a: u32,
    register_b: u32,
    register_c: u32,
    combo: u32,
    op: u32,
    instruction_pointer: u32,
    jumped: bool,
}

impl Computer {
    fn get_combo_operand(&mut self) -> u32 {
        match self.combo {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            _ => {
                panic!("reserved operand")
            }
        }
    }

    fn run(&mut self) {
        match self.op {
            0 => {
                self.register_a /= 2_u32.pow(self.get_combo_operand());
            }
            1 => {
                self.register_b ^= self.combo;
            }
            2 => {
                self.register_b = self.get_combo_operand() % 8;
            }
            3 => match self.register_a {
                0 => {}
                _ => {
                    self.instruction_pointer = self.combo;
                    self.jumped = true;
                }
            },
            4 => {
                self.register_b ^= self.register_c;
            }
            5 => {
                print!("{},", self.get_combo_operand() % 8);
            }
            6 => {
                self.register_b = self.register_a / 2_u32.pow(self.get_combo_operand());
            }
            7 => {
                self.register_c = self.register_a / 2_u32.pow(self.get_combo_operand());
            }
            _ => {
                panic!("unknown opcode")
            }
        }
    }
}

fn solve_p1(computer: &mut Computer, instructions: &[u32]) {
    loop {
        if computer.instruction_pointer as usize >= instructions.len() {
            break;
        }

        computer.op = instructions[computer.instruction_pointer as usize];
        computer.combo = instructions[computer.instruction_pointer as usize + 1];

        computer.run();

        if !computer.jumped {
            computer.instruction_pointer += 2;
        } else {
            computer.jumped = false;
        }
    }
}

fn main() {
    let input = read_to_string("input.txt").unwrap();

    let mut computer = Computer {
        register_a: 0,
        register_b: 0,
        register_c: 0,
        combo: 0,
        op: 0,
        instruction_pointer: 0,
        jumped: false,
    };

    let mut instructions: Vec<u32> = Vec::new();

    input.lines().for_each(|line| {
        if line.contains("Register A") {
            computer.register_a = line.split(": ").last().unwrap().parse().unwrap();
        } else if line.contains("Register B") {
            computer.register_b = line.split(": ").last().unwrap().parse().unwrap();
        } else if line.contains("Register C") {
            computer.register_c = line.split(": ").last().unwrap().parse().unwrap();
        } else if line.contains("Program") {
            instructions = line
                .split(": ")
                .last()
                .unwrap()
                .split(",")
                .collect::<Vec<&str>>()
                .iter()
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
        }
    });

    solve_p1(&mut computer, &instructions);

    println!();
    println!("{:?}", computer);
}
