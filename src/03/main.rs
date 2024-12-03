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
    let characters = input.chars().collect::<Vec<char>>();

    let mut enabled = true;
    let enabled_s = characters
        .iter()
        .enumerate()
        .filter_map(|(i, c)| {
            if i < characters.len() - 7 {
                let upcoming = characters[i..i + 7].iter().collect::<String>();

                if upcoming == "don't()" {
                    enabled = false;
                };

                if upcoming.starts_with("do()") {
                    enabled = true;
                };
            }

            if enabled {
                Some(c)
            } else {
                None
            }
        })
        .collect::<String>();

    Regex::new(r"mul\((\d+),(\d+)\)")
        .unwrap()
        .captures_iter(&enabled_s)
        .map(|cap| {
            cap.get(1).unwrap().as_str().parse::<i32>().unwrap()
                * cap.get(2).unwrap().as_str().parse::<i32>().unwrap()
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
