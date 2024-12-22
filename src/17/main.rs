use std::{collections::VecDeque, time::Instant};

fn main() {
    println!("--- Day 17: Chronospatial Computer ---");
    let start: Instant = Instant::now();

    let input: &str = include_str!("./input.txt");
    let (registers, program) = parse(input);
    println!("pt1: {}", pt1(registers, &program));
    println!("pt2: {}", pt2(&program));
    println!("Execution time: {:.2?}", start.elapsed());
}

fn parse(input: &str) -> ((u64, u64, u64), Vec<u8>) {
    input
        .split_once("\n\n")
        .map(|(l, r)| {
            let registers: Vec<u64> = l
                .lines()
                .map(|line| line.split_once(": ").unwrap().1.parse::<u64>().unwrap())
                .collect();

            let program: Vec<u8> = r
                .trim()
                .split_once(": ")
                .unwrap()
                .1
                .split(",")
                .map(|d| d.parse::<u8>().unwrap())
                .collect();

            ((registers[0], registers[1], registers[2]), program)
        })
        .unwrap()
}

fn pt1(registers: (u64, u64, u64), program: &Vec<u8>) -> String {
    let output = run_program(registers, program);

    output
        .iter()
        .map(|d| d.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

fn pt2(program: &Vec<u8>) -> u64 {
    let mut queue: VecDeque<(usize, u64)> = VecDeque::from([(0, 0)]);

    let mut ans = 0;
    while let Some((index, curr_a)) = queue.pop_front() {
        if index == program.len() {
            ans = curr_a;
            break;
        }

        (0..8).for_each(|i| {
            let next_a = (curr_a << 3) | i;
            let output = run_program((next_a, 0, 0), &program.clone());
            if let Some(out) = output.iter().nth_back(index) {
                if program[program.len() - 1 - index] == *out {
                    queue.push_back((index + 1, next_a));
                }
            }
        });
    }

    ans
}

fn run_program(mut registers: (u64, u64, u64), program: &[u8]) -> Vec<u8> {
    let mut output: Vec<u8> = vec![];
    let mut instruction_pointer = 0;

    while instruction_pointer < program.len() {
        let opcode = program[instruction_pointer];
        let literal_operand = program[instruction_pointer + 1];
        let combo_operand = match literal_operand {
            n if n <= 3 => n as u64,
            4 => registers.0,
            5 => registers.1,
            6 => registers.2,
            7 => u64::MAX,
            _ => unreachable!(),
        };
        let mut instruction_pointer_jumped = false;

        match opcode {
            0 => {
                let div: f32 = (registers.0 / 2_u64.pow(combo_operand as u32)) as f32;
                registers.0 = div.trunc() as u64;
            }
            1 => {
                registers.1 ^= literal_operand as u64;
            }
            2 => {
                registers.1 = combo_operand % 8;
            }
            3 => {
                if registers.0 != 0 {
                    instruction_pointer = literal_operand as usize;
                    instruction_pointer_jumped = true;
                }
            }
            4 => {
                registers.1 ^= registers.2;
            }
            5 => {
                output.push((combo_operand % 8) as u8);
            }
            6 => {
                let div: f64 = (registers.0 / 2_u64.pow(combo_operand as u32)) as f64;
                registers.1 = div.trunc() as u64;
            }
            7 => {
                let div: f64 = (registers.0 / 2_u64.pow(combo_operand as u32)) as f64;
                registers.2 = div.trunc() as u64;
            }
            _ => unreachable!(),
        }

        if !instruction_pointer_jumped {
            instruction_pointer += 2;
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let input = include_str!("./example_pt1.txt");
        let (registers, program) = parse(input);
        let result = pt1(registers, &program);
        assert_eq!(result, "4,6,3,5,6,3,5,2,1,0".to_string());
    }

    #[test]
    fn pt2_test() {
        let input = include_str!("./example_pt2.txt");
        let (_, program) = parse(input);
        let result = pt2(&program);
        assert_eq!(result, 117440);
    }
}
