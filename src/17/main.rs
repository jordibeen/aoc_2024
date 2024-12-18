use std::time::Instant;

fn main() {
    println!("--- Day 17: Chronospatial Computer ---");
    let start: Instant = Instant::now();

    let input: &str = include_str!("./input.txt");
    println!("pt1: {}", pt1(&input));
    println!("Execution time: {:.2?}", start.elapsed());
}

fn pt1(input: &str) -> String {
    let (mut registers, program) = input
        .split_once("\n\n")
        .map(|(l, r)| {
            let registers: Vec<u32> = l
                .lines()
                .map(|line| line.split_once(": ").unwrap().1.parse::<u32>().unwrap())
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
        .unwrap();

    let mut output: Vec<u8> = vec![];
    let mut instruction_pointer = 0;
    while instruction_pointer < program.len() {
        let opcode = program[instruction_pointer];
        let literal_operand = program[instruction_pointer + 1];
        let combo_operand: u32 = match literal_operand {
            n if n <= 3 => n as u32,
            4 => registers.0,
            5 => registers.1,
            6 => registers.2,
            7 => u32::MAX,
            _ => unreachable!(),
        };
        let mut instruction_pointer_jumped = false;

        match opcode {
            0 => {
                let div: f32 = (registers.0 / 2_u32.pow(combo_operand)) as f32;
                registers.0 = div.trunc() as u32;
            }
            1 => {
                registers.1 = registers.1 ^ literal_operand as u32;
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
                registers.1 = registers.1 ^ registers.2;
            }
            5 => {
                output.push((combo_operand % 8) as u8);
            }
            6 => {
                let div: f32 = (registers.0 / 2_u32.pow(combo_operand)) as f32;
                registers.1 = div.trunc() as u32;
            }
            7 => {
                let div: f32 = (registers.0 / 2_u32.pow(combo_operand)) as f32;
                registers.2 = div.trunc() as u32;
            }
            _ => unreachable!(),
        }

        if !instruction_pointer_jumped {
            instruction_pointer += 2;
        }
    }

    output
        .iter()
        .map(|d| d.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let input = include_str!("./example.txt");
        let result = pt1(&input);
        assert_eq!(result, "4,6,3,5,6,3,5,2,1,0".to_string());
    }
}
