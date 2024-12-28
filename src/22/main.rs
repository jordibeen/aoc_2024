use std::time::Instant;

fn main() {
    println!("--- Day 22: Monkey Market ---");
    let start: Instant = Instant::now();

    let input: &str = include_str!("./input.txt");
    println!("pt1: {}", pt1(&input));
    println!("Execution time: {:.2?}", start.elapsed());
}

fn pt1(input: &str) -> i64 {
    input
        .lines()
        .map(|line| generate(line.parse::<i64>().unwrap(), 2000))
        .sum()
}

fn generate(mut secret_number: i64, times: usize) -> i64 {
    (0..times).for_each(|_| {
        secret_number = (secret_number ^ (secret_number * 64)) % 16777216;
        secret_number = (secret_number ^ ((secret_number / 32) as f64).floor() as i64) % 16777216;
        secret_number = (secret_number ^ secret_number * 2048) % 16777216;
    });

    secret_number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let input = include_str!("./example.txt");
        let result = pt1(&input);
        assert_eq!(result, 37327623);
    }
}
