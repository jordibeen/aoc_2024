use regex::Regex;
use std::time::Instant;

fn main() {
    println!("--- Day 1: Mull It Over ---");

    let input: &str = include_str!("./input.txt");

    let start: Instant = Instant::now();
    println!("pt1: {} (finished in {:.2?})", pt1(&input), start.elapsed());

    let start: Instant = Instant::now();
    println!("pt2: {} (finished in {:.2?})", pt2(&input), start.elapsed());
}

fn pt1(input: &str) -> i32 {
    Regex::new(r"mul\((\d+),(\d+)\)")
        .unwrap()
        .captures_iter(input)
        .map(|cap| {
            cap.get(1).unwrap().as_str().parse::<i32>().unwrap()
                * cap.get(2).unwrap().as_str().parse::<i32>().unwrap()
        })
        .sum()
}

fn pt2(input: &str) -> i32 {
    let mut enabled = true;
    Regex::new(r"don't\(\)|do\(\)|(mul\((\d+),(\d+)\))")
        .unwrap()
        .captures_iter(input)
        .map(|cap| {
            match cap.get(0).unwrap().as_str() {
                "don't()" => enabled = false,
                "do()" => enabled = true,
                _ => {
                    if enabled {
                        return cap.get(2).unwrap().as_str().parse::<i32>().unwrap()
                            * cap.get(3).unwrap().as_str().parse::<i32>().unwrap();
                    }
                }
            }
            0
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let input = include_str!("./example_pt1.txt");
        let result = pt1(&input);
        assert_eq!(result, 161);
    }

    #[test]
    fn pt2_test() {
        let input = include_str!("./example_pt2.txt");
        let result = pt2(&input);
        assert_eq!(result, 48);
    }
}
