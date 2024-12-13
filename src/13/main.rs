use regex::Regex;
use std::time::Instant;

#[derive(Debug)]
struct Machine {
    button_a: (i32, i32),
    button_b: (i32, i32),
    prize: (i32, i32),
}

impl Machine {
    fn from_input(input: &str) -> Self {
        let parsed: Vec<(i32, i32)> = input
            .lines()
            .map(|line| {
                let m: Vec<i32> = Regex::new(r"(\d+)")
                    .unwrap()
                    .captures_iter(line)
                    .map(|cap| cap.get(0).unwrap().as_str().parse::<i32>().unwrap())
                    .collect();

                (m[0], m[1])
            })
            .collect();

        Machine {
            button_a: parsed[0],
            button_b: parsed[1],
            prize: parsed[2],
        }
    }
}

fn main() {
    println!("--- Day 13: Claw Contraption ---");
    let start: Instant = Instant::now();

    let input: &str = include_str!("./input.txt");
    println!("pt1: {}", pt1(&input));
    println!("Execution time: {:.2?}", start.elapsed());
}

fn pt1(input: &str) -> i32 {
    let machines: Vec<Machine> = input
        .split("\n\n")
        .map(|m| Machine::from_input(m))
        .collect();

    machines
        .iter()
        .filter_map(|machine| {
            let mut tokens: Option<i32> = None;

            let mut b_presses = 0;
            let mut curr = machine.prize;
            while curr.0 >= 0 && curr.1 >= 0 && b_presses <= 100 {
                b_presses += 1;
                curr = ((curr.0 - machine.button_b.0), (curr.1 - machine.button_b.1));
                if curr.0 % machine.button_a.0 == 0
                    && curr.1 % machine.button_a.1 == 0
                    && curr.0 / machine.button_a.0 == curr.1 / machine.button_a.1
                {
                    let a_presses = curr.0 / machine.button_a.0;
                    if a_presses <= 100 {
                        tokens = Some((a_presses * 3) + b_presses);
                    }
                }
            }

            tokens
        })
        .sum::<i32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let input = include_str!("./example.txt");
        let result = pt1(&input);
        assert_eq!(result, 480);
    }
}
