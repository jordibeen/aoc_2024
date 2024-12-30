use std::time::Instant;

fn main() {
    println!("--- Day 25: Code Chronicle ---");
    let start: Instant = Instant::now();

    let input: &str = include_str!("./input.txt");
    println!("pt1: {}", pt1(&input));
    println!("Execution time: {:.2?}", start.elapsed());
}

fn pt1(input: &str) -> i32 {
    let (m, n) = (7, 5);
    let (locks, keys): (Vec<Vec<u8>>, Vec<Vec<u8>>) =
        input
            .split("\n\n")
            .fold((vec![], vec![]), |(mut locks, mut keys), lines| {
                let schematic = lines
                    .lines()
                    .map(|line| line.chars().map(|c| c == '#').collect::<Vec<bool>>())
                    .collect::<Vec<Vec<bool>>>();

                let pin_heights = (0..n)
                    .map(|y| (0..m).filter(|x| schematic[*x][y]).count() as u8 - 1)
                    .collect::<Vec<u8>>();

                if schematic[0].iter().all(|v| *v) {
                    locks.push(pin_heights);
                } else {
                    keys.push(pin_heights);
                }

                (locks, keys)
            });

    keys.iter()
        .map(|key| {
            locks
                .iter()
                .filter(|lock| {
                    key.iter()
                        .zip(lock.iter())
                        .all(|(key_part, lock_part)| (key_part + lock_part) < m as u8 - 1)
                })
                .count() as i32
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let input = include_str!("./example.txt");
        let result = pt1(&input);
        assert_eq!(result, 3);
    }
}
