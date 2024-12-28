use std::collections::{HashMap, HashSet};
use std::time::Instant;

fn main() {
    println!("--- Day 22: Monkey Market ---");
    let start: Instant = Instant::now();

    let input: &str = include_str!("./input.txt");
    println!("pt1: {}", pt1(&input));
    println!("pt2: {}", pt2(&input));
    println!("Execution time: {:.2?}", start.elapsed());
}

fn pt1(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let mut secret_number = line.parse::<i64>().unwrap();
            (0..2000).for_each(|_| {
                secret_number = generate(secret_number);
            });
            secret_number
        })
        .sum()
}

fn pt2(input: &str) -> i32 {
    let mut sequences: HashMap<Vec<i32>, i32> = HashMap::new();
    input.lines().for_each(|line| {
        let mut secret_number = line.parse::<i32>().unwrap();
        let mut seen: HashSet<Vec<i32>> = HashSet::new();

        (0..2000).fold(
            (vec![], secret_number % 10),
            |(mut price_changes, prev_price), _| {
                secret_number = generate(secret_number as i64) as i32;
                let price = secret_number % 10;

                price_changes.push(price - prev_price);

                let sequence = price_changes
                    .iter()
                    .rev()
                    .take(4)
                    .map(|v| *v)
                    .collect::<Vec<i32>>();

                if !seen.contains(&sequence) {
                    sequences
                        .entry(sequence.clone())
                        .and_modify(|v| *v += price)
                        .or_insert(price);
                    seen.insert(sequence);
                }

                (price_changes, price)
            },
        );
    });

    *sequences.values().max().unwrap()
}

fn generate(mut secret_number: i64) -> i64 {
    secret_number = (secret_number ^ (secret_number * 64)) % 16777216;
    secret_number = (secret_number ^ ((secret_number / 32) as f64).floor() as i64) % 16777216;
    secret_number = (secret_number ^ secret_number * 2048) % 16777216;
    secret_number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let input = include_str!("./example_pt1.txt");
        let result = pt1(&input);
        assert_eq!(result, 37327623);
    }

    #[test]
    fn pt2_test() {
        let input = include_str!("./example_pt2.txt");
        let result = pt2(&input);
        assert_eq!(result, 23);
    }
}
