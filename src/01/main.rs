use std::collections::HashMap;
use std::time::Instant;

fn main() {
    println!("--- Day 1: Historian Hysteria ---");
    let start: Instant = Instant::now();

    let input: &str = include_str!("./input.txt");
    println!("pt1: {} (finished in {:.2?})", pt1(&input), start.elapsed());
    println!("pt2: {} (finished in {:.2?})", pt2(&input), start.elapsed());
}

fn pt1(input: &str) -> i32 {
    let (mut left, mut right) = input.lines().fold(
        (vec![], vec![]),
        |(mut left, mut right): (Vec<i32>, Vec<i32>), line| {
            let (l, r) = line.split_once("   ").unwrap();
            left.push(l.parse::<i32>().unwrap());
            right.push(r.parse::<i32>().unwrap());

            (left, right)
        },
    );

    left.sort_unstable();
    right.sort_unstable();

    left.iter()
        .zip(right.iter())
        .map(|(l, r)| l.abs_diff(*r) as i32)
        .sum::<i32>()
}

fn pt2(input: &str) -> i32 {
    let (left, counts) = input.lines().fold(
        (vec![], HashMap::new()),
        |(mut left, mut counts): (Vec<i32>, HashMap<i32, i32>), line| {
            let (l, r) = line.split_once("   ").unwrap();
            left.push(l.parse::<i32>().unwrap());
            counts
                .entry(r.parse::<i32>().unwrap())
                .and_modify(|v| *v += 1)
                .or_insert(1);

            (left, counts)
        },
    );

    left.iter()
        .map(|l| {
            if let Some(count) = counts.get(l) {
                l * count
            } else {
                0
            }
        })
        .sum::<i32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let input = include_str!("./example_pt1.txt");
        let result = pt1(&input);
        assert_eq!(result, 11);
    }

    #[test]
    fn pt2_test() {
        let input = include_str!("./example_pt2.txt");
        let result = pt2(&input);
        assert_eq!(result, 31);
    }
}
