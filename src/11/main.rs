use std::collections::HashMap;
use std::time::Instant;

fn main() {
    println!("--- Day 11: Plutonian Pebbles ---");
    let start: Instant = Instant::now();

    let input: &str = include_str!("./input.txt");
    let stones = parse(&input);
    println!("pt1: {}", blink(stones.clone(), 25));
    println!("pt2: {}", blink(stones.clone(), 75));
    println!("Execution time: {:.2?}", start.elapsed());
}

fn parse(input: &str) -> HashMap<i64, i64> {
    input
        .split_whitespace()
        .fold(HashMap::new(), |mut stones, c| {
            stones
                .entry(c.parse::<i64>().unwrap())
                .and_modify(|v| *v += 1)
                .or_insert(1);
            stones
        })
}

fn blink(mut stones: HashMap<i64, i64>, times: i32) -> i64 {
    (0..times).for_each(|_| {
        let mut next_stones: HashMap<i64, i64> = HashMap::new();
        for (stone, count) in stones.iter() {
            if stone == &0 {
                next_stones
                    .entry(1)
                    .and_modify(|v| *v += *count)
                    .or_insert(*count);
                continue;
            }

            let digits = stone.to_string();
            if digits.len() % 2 == 0 {
                let (l, r) = digits.split_at(digits.len() / 2);
                next_stones
                    .entry(l.parse::<i64>().unwrap())
                    .and_modify(|v| *v += *count)
                    .or_insert(*count);
                next_stones
                    .entry(r.parse::<i64>().unwrap())
                    .and_modify(|v| *v += *count)
                    .or_insert(*count);
                continue;
            }

            next_stones
                .entry(stone * 2024)
                .and_modify(|v| *v += *count)
                .or_insert(*count);
        }

        stones = next_stones;
    });

    stones.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let input = include_str!("./example.txt");
        let stones = parse(&input);
        let result = blink(stones, 25);
        assert_eq!(result, 55312);
    }

    #[test]
    fn pt2_test() {
        let input = include_str!("./example.txt");
        let stones = parse(&input);
        let result = blink(stones, 75);
        assert_eq!(result, 65601038650482);
    }
}
