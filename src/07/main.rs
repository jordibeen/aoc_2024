use std::collections::VecDeque;
use std::time::Instant;

fn main() {
    println!("--- Day 7: Bridge Repair ---");
    let start: Instant = Instant::now();

    let input: &str = include_str!("./input.txt");
    println!("pt1: {} (finished in {:.2?})", pt1(&input), start.elapsed());
}

fn pt1(input: &str) -> i64 {
    input
        .lines()
        .filter_map(|line| {
            let (test_value, numbers) = line
                .split_once(": ")
                .map(|(test_value, numbers)| {
                    (
                        test_value.parse::<i64>().unwrap(),
                        numbers
                            .split(" ")
                            .map(|number| number.parse::<i64>().unwrap())
                            .collect::<Vec<i64>>(),
                    )
                })
                .unwrap();

            let mut queue: VecDeque<(i64, &Vec<i64>, usize, i64)> =
                VecDeque::from([(test_value, &numbers, 0, numbers[0])]);

            while let Some((test_value, numbers, i, curr_value)) = queue.pop_front() {
                if i != numbers.len() - 1 {
                    queue.push_back((test_value, numbers, i + 1, curr_value + numbers[i + 1]));
                    queue.push_back((test_value, numbers, i + 1, curr_value * numbers[i + 1]));
                } else {
                    if test_value == curr_value {
                        return Some(test_value);
                    }
                }
            }

            None
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
        assert_eq!(result, 3749);
    }
}
