use regex::Regex;
use std::time::Instant;

#[derive(Debug)]
struct Machine {
    button_a: (i32, i32),
    button_b: (i32, i32),
    prize_pt1: (i32, i32),
    prize_pt2: (i64, i64),
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
            prize_pt1: parsed[2],
            prize_pt2: (
                parsed[2].0 as i64 + 10000000000000,
                parsed[2].1 as i64 + 10000000000000,
            ),
        }
    }
}

fn main() {
    println!("--- Day 13: Claw Contraption ---");
    let start: Instant = Instant::now();

    let input: &str = include_str!("./input.txt");
    println!("pt1: {}", pt1(&input));
    println!("pt2: {}", pt2(&input));
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
            let mut curr = machine.prize_pt1;
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

fn pt2(input: &str) -> i64 {
    let machines: Vec<Machine> = input
        .split("\n\n")
        .map(|m| Machine::from_input(m))
        .collect();

    machines
        .iter()
        .filter_map(|machine| {
            let mut tokens: Option<i64> = None;

            let det =
                machine.button_a.0 * machine.button_b.1 - machine.button_a.1 * machine.button_b.0;

            if det != 0 {
                let det_x = machine.prize_pt2.0 * machine.button_b.1 as i64
                    - machine.prize_pt2.1 * machine.button_b.0 as i64;
                let det_y = machine.button_a.0 as i64 * machine.prize_pt2.1
                    - machine.button_a.1 as i64 * machine.prize_pt2.0;

                let intersection = (det_x as f64 / det as f64, det_y as f64 / det as f64);

                if intersection.0.fract() == 0.0 && intersection.1.fract() == 0.0 {
                    let (a_presses, b_presses) = (intersection.0 as i64, intersection.1 as i64);
                    tokens = Some((a_presses * 3) + b_presses);
                }
            }

            tokens
        })
        .sum::<i64>()
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

    #[test]
    fn pt2_test() {
        let input = include_str!("./example.txt");
        let result = pt2(&input);
        assert_eq!(result, 875318608908);
    }
}
