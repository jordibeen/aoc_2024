use std::collections::{HashMap, VecDeque};
use std::time::Instant;

fn main() {
    println!("--- Day 19: Linen Layout ---");
    let start: Instant = Instant::now();

    let input: &str = include_str!("./input.txt");
    let (towels, designs) = parse(&input);
    println!("pt1: {}", pt1(&towels, &designs));
    println!("pt2: {}", pt2(&towels, &designs));
    println!("Execution time: {:.2?}", start.elapsed());
}
fn parse(input: &str) -> (Vec<String>, Vec<String>) {
    input
        .split_once("\n\n")
        .map(|(t, d)| {
            (
                t.split(", ")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>(),
                d.lines().map(|s| s.to_string()).collect::<Vec<String>>(),
            )
        })
        .unwrap()
}

fn pt1(towels: &Vec<String>, designs: &Vec<String>) -> i32 {
    designs
        .iter()
        .filter(|design| {
            let mut queue: VecDeque<String> = VecDeque::from([design.to_string()]);
            while let Some(leftover) = queue.pop_front() {
                if leftover.is_empty() {
                    return true;
                }

                towels.iter().for_each(|towel| {
                    if leftover.starts_with(towel) {
                        queue.push_front(leftover.trim_start_matches(towel).to_string());
                    }
                });
            }
            false
        })
        .count() as i32
}

fn pt2(towels: &Vec<String>, designs: &Vec<String>) -> i64 {
    let cache = &mut HashMap::new();
    designs
        .iter()
        .map(|design| ways(cache, &towels, design.to_string()))
        .sum()
}

fn ways(cache: &mut HashMap<String, i64>, towels: &Vec<String>, leftover: String) -> i64 {
    if let Some(cached_val) = cache.get(&leftover) {
        return *cached_val;
    }

    let mut ans = 0;
    if leftover.is_empty() {
        ans = 1;
    }

    for towel in towels.iter() {
        if leftover.starts_with(towel) {
            ans += ways(cache, &towels, leftover.chars().skip(towel.len()).collect());
        }
    }

    cache.insert(leftover, ans);

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let input = include_str!("./example.txt");
        let (towels, designs) = parse(&input);
        let result = pt1(&towels, &designs);
        assert_eq!(result, 6);
    }

    #[test]
    fn pt2_test() {
        let input = include_str!("./example.txt");
        let (towels, designs) = parse(&input);
        let result = pt2(&towels, &designs);
        assert_eq!(result, 16);
    }
}
