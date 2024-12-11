use std::time::Instant;

fn main() {
    println!("--- Day 11: Plutonian Pebbles ---");
    let start: Instant = Instant::now();

    let input: &str = include_str!("./input.txt");
    let pt1 = pt1(&input);
    println!("pt1: {}", pt1);
    println!("Execution time: {:.2?}", start.elapsed());
}

fn pt1(input: &str) -> i32 {
    let mut stones: Vec<i64> = input
        .split_whitespace()
        .map(|c| c.parse::<i64>().unwrap())
        .collect();

    (0..25).for_each(|_| {
        let mut i = 0;
        while i < stones.len() {
            if stones[i] == 0 {
                stones[i] = 1;
                i += 1;
                continue;
            }
            let digits = stones[i].to_string();
            if digits.len() % 2 == 0 {
                let (l, r) = digits.split_at(digits.len() / 2);
                stones[i] = l.parse::<i64>().unwrap();
                stones.insert(i + 1, r.parse::<i64>().unwrap());
                i += 2;
                continue;
            }
            stones[i] = stones[i] * 2024;
            i += 1;
        }
    });

    stones.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let input = include_str!("./example.txt");
        let result = pt1(&input);
        assert_eq!(result, 55312);
    }
}
