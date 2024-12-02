use std::time::Instant;

fn main() {
    println!("--- Day 2: Red-Nosed Reports ---");
    let start: Instant = Instant::now();

    let input: &str = include_str!("./input.txt");
    println!("pt1: {} (finished in {:.2?})", pt1(&input), start.elapsed());
}

fn pt1(input: &str) -> i32 {
    input
        .lines()
        .filter(|line| {
            let levels: Vec<i32> = line
                .split(" ")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            let ord = levels[0].cmp(&levels[1]);

            for (i, level) in levels.iter().enumerate() {
                if let Some(next) = levels.get(i + 1) {
                    if level.cmp(&next) != ord || level.abs_diff(*next) > 3 {
                        return false;
                    }
                };
            }

            true
        })
        .count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let input = include_str!("./example_pt1.txt");
        let result = pt1(&input);
        assert_eq!(result, 2);
    }
}
