use std::collections::VecDeque;
use std::time::Instant;

fn main() {
    println!("--- Day 19: Linen Layout ---");
    let start: Instant = Instant::now();

    let input: &str = include_str!("./input.txt");
    println!("pt1: {}", pt1(&input));
    println!("Execution time: {:.2?}", start.elapsed());
}

fn pt1(input: &str) -> i32 {
    let (towels, designs) = input
        .split_once("\n\n")
        .map(|(t, d)| {
            (
                t.split(", ")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>(),
                d.lines().map(|s| s.to_string()).collect::<Vec<String>>(),
            )
        })
        .unwrap();

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let input = include_str!("./example.txt");
        let result = pt1(&input);
        assert_eq!(result, 6);
    }
}
