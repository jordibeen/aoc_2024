use std::collections::HashMap;
use std::time::Instant;

fn main() {
    println!("--- Day 5: Print Queue ---");
    let start: Instant = Instant::now();

    let input: &str = include_str!("./input.txt");
    println!("pt1: {} (finished in {:.2?})", pt1(&input), start.elapsed());
}

fn pt1(input: &str) -> i32 {
    let (page_ordering, updates) = input.split_once("\n\n").unwrap();

    let (nexts, previouses): (HashMap<i32, Vec<i32>>, HashMap<i32, Vec<i32>>) =
        page_ordering.lines().fold(
            (HashMap::new(), HashMap::new()),
            |(mut nexts, mut previouses), line| {
                let (l, r) = line
                    .split_once("|")
                    .map(|(l, r)| (l.parse::<i32>().unwrap(), r.parse::<i32>().unwrap()))
                    .unwrap();

                nexts.entry(l).and_modify(|v| v.push(r)).or_insert(vec![r]);

                previouses
                    .entry(r)
                    .and_modify(|v| v.push(l))
                    .or_insert(vec![l]);

                (nexts, previouses)
            },
        );

    updates
        .lines()
        .map(|line| {
            let numbers: Vec<i32> = line.split(",").map(|n| n.parse::<i32>().unwrap()).collect();
            let mut correct = true;

            for (number, next) in numbers.iter().zip(numbers.iter().skip(1)) {
                if let Some(nexts) = nexts.get(&number) {
                    if !nexts.contains(&next) {
                        correct = false;
                    }
                }

                if let Some(previouses) = previouses.get(&next) {
                    if !previouses.contains(&number) {
                        correct = false;
                    }
                }
            }

            if correct {
                numbers[numbers.len() / 2]
            } else {
                0
            }
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
        assert_eq!(result, 143);
    }
}
