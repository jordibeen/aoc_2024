use std::collections::{HashMap, VecDeque};
use std::time::Instant;

fn main() {
    println!("--- Day 5: Print Queue ---");
    let start: Instant = Instant::now();
    let input: &str = include_str!("./input.txt");

    let (afters, befores, updates) = parse(&input);
    println!("pt1: {}", pt1(&afters, &befores, &updates));
    println!("pt2: {}", pt2(&afters, &befores, &updates));
    println!("Execution time: {:.2?}", start.elapsed());
}

fn parse(
    input: &str,
) -> (
    HashMap<i32, Vec<i32>>,
    HashMap<i32, Vec<i32>>,
    Vec<Vec<i32>>,
) {
    let (page_ordering, updates) = input.split_once("\n\n").unwrap();

    let (afters, befores): (HashMap<i32, Vec<i32>>, HashMap<i32, Vec<i32>>) =
        page_ordering.lines().fold(
            (HashMap::new(), HashMap::new()),
            |(mut afters, mut befores), line| {
                let (l, r) = line
                    .split_once("|")
                    .map(|(l, r)| (l.parse::<i32>().unwrap(), r.parse::<i32>().unwrap()))
                    .unwrap();

                afters.entry(l).and_modify(|v| v.push(r)).or_insert(vec![r]);

                befores
                    .entry(r)
                    .and_modify(|v| v.push(l))
                    .or_insert(vec![l]);

                (afters, befores)
            },
        );

    let updates = updates
        .lines()
        .map(|line| line.split(",").map(|n| n.parse::<i32>().unwrap()).collect())
        .collect();

    (afters, befores, updates)
}

fn pt1(
    afters: &HashMap<i32, Vec<i32>>,
    befores: &HashMap<i32, Vec<i32>>,
    updates: &Vec<Vec<i32>>,
) -> i32 {
    updates
        .iter()
        .map(|numbers| {
            if check_correctness(afters, befores, numbers).0 {
                numbers[numbers.len() / 2]
            } else {
                0
            }
        })
        .sum()
}

fn pt2(
    afters: &HashMap<i32, Vec<i32>>,
    befores: &HashMap<i32, Vec<i32>>,
    updates: &Vec<Vec<i32>>,
) -> i32 {
    let incorrects: Vec<&Vec<i32>> = updates
        .iter()
        .filter_map(|numbers| {
            if !check_correctness(afters, befores, &numbers).0 {
                Some(numbers)
            } else {
                None
            }
        })
        .collect();

    incorrects
        .iter()
        .filter_map(|incorrect| {
            let mut queue: VecDeque<Vec<i32>> = VecDeque::from([incorrect.to_vec()]);
            while let Some(numbers) = queue.pop_front() {
                let (is_correct, faulty_element_i) = check_correctness(afters, befores, &numbers);

                if is_correct {
                    return Some(numbers[numbers.len() / 2]);
                }

                let mut permutation = numbers.to_vec();
                let faulty_element = permutation.remove(faulty_element_i);
                permutation.push(faulty_element);

                queue.push_back(permutation.to_vec())
            }

            None
        })
        .sum()
}

fn check_correctness(
    afters: &HashMap<i32, Vec<i32>>,
    befores: &HashMap<i32, Vec<i32>>,
    numbers: &Vec<i32>,
) -> (bool, usize) {
    for (i, number) in numbers.iter().enumerate() {
        if let Some(afters) = afters.get(&number) {
            if !numbers[i + 1..].iter().all(|v| afters.contains(v)) {
                return (false, i);
            }
        } else {
            if i != numbers.len() - 1 {
                return (false, i);
            }
        }

        if let Some(befores) = befores.get(&number) {
            if !numbers[..i].iter().all(|v| befores.contains(v)) {
                return (false, i);
            }
        } else {
            if i != 0 {
                return (false, i);
            }
        }
    }

    (true, usize::MAX)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let input = include_str!("./example.txt");
        let (afters, befores, updates) = parse(&input);
        let result = pt1(&afters, &befores, &updates);
        assert_eq!(result, 143);
    }

    #[test]
    fn pt2_test() {
        let input = include_str!("./example.txt");
        let (afters, befores, updates) = parse(&input);
        let result = pt2(&afters, &befores, &updates);
        assert_eq!(result, 123);
    }
}
