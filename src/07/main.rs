use std::collections::VecDeque;
use std::time::Instant;

fn main() {
    println!("--- Day 7: Bridge Repair ---");
    let start: Instant = Instant::now();

    let input: &str = include_str!("./input.txt");
    let equations = parse(&input);
    println!("pt1: {}", pt1(&equations));
    println!("pt2: {}", pt2(&equations));
    println!("Execution time: {:.2?}", start.elapsed());
}

fn parse(input: &str) -> Vec<(i64, Vec<i64>)> {
    input
        .lines()
        .map(|line| {
            line.split_once(": ")
                .map(|(test_value, numbers)| {
                    (
                        test_value.parse::<i64>().unwrap(),
                        numbers
                            .split(" ")
                            .map(|number| number.parse::<i64>().unwrap())
                            .collect::<Vec<i64>>(),
                    )
                })
                .unwrap()
        })
        .collect()
}

fn pt1(equations: &Vec<(i64, Vec<i64>)>) -> i64 {
    equations
        .iter()
        .filter_map(|(test_value, numbers)| {
            let mut queue: VecDeque<(&Vec<i64>, usize, i64)> =
                VecDeque::from([(numbers, 0, numbers[0])]);

            while let Some((numbers, i, curr_value)) = queue.pop_front() {
                if i != numbers.len() - 1 {
                    queue.push_back((numbers, i + 1, curr_value + numbers[i + 1]));
                    queue.push_back((numbers, i + 1, curr_value * numbers[i + 1]));
                } else {
                    if test_value == &curr_value {
                        return Some(test_value);
                    }
                }
            }

            None
        })
        .sum()
}

fn pt2(equations: &Vec<(i64, Vec<i64>)>) -> i64 {
    equations
        .iter()
        .filter_map(|(test_value, numbers)| {
            let mut queue: VecDeque<(&Vec<i64>, usize, i64)> =
                VecDeque::from([(numbers, 0, numbers[0])]);

            while let Some((numbers, i, curr_value)) = queue.pop_front() {
                if i != numbers.len() - 1 {
                    queue.push_back((numbers, i + 1, curr_value + numbers[i + 1]));
                    queue.push_back((numbers, i + 1, curr_value * numbers[i + 1]));
                    queue.push_back((
                        numbers,
                        i + 1,
                        format!("{}{}", curr_value, numbers[i + 1])
                            .parse::<i64>()
                            .unwrap(),
                    ));
                } else {
                    if test_value == &curr_value {
                        return Some(test_value);
                    }
                }
            }

            None
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let input = include_str!("./example.txt");
        let equations = parse(&input);
        let result = pt1(&equations);
        assert_eq!(result, 3749);
    }

    #[test]
    fn pt2_test() {
        let input = include_str!("./example.txt");
        let equations = parse(&input);
        let result = pt2(&equations);
        assert_eq!(result, 11387);
    }
}
