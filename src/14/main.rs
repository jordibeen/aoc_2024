use regex::Regex;
use std::time::Instant;

fn main() {
    println!("--- Day 14: Restroom Redoubt ---");
    let start: Instant = Instant::now();

    let input: &str = include_str!("./input.txt");
    println!("pt1: {}", pt1(&input, (101, 103)));
    println!("Execution time: {:.2?}", start.elapsed());
}

fn pt1(input: &str, size: (isize, isize)) -> i32 {
    let (width, height) = size;
    let final_positions: Vec<(isize, isize)> = input
        .lines()
        .map(|line| {
            let digits: Vec<isize> = Regex::new(r"(-?\d+)")
                .unwrap()
                .captures_iter(line)
                .map(|cap| cap.get(0).unwrap().as_str().parse::<isize>().unwrap())
                .collect();

            let position: (isize, isize) = (digits[0], digits[1]);
            let velocity: (isize, isize) = (digits[2], digits[3]);

            let mut final_position = (
                ((position.0 + velocity.0 * 100) % width),
                ((position.1 + velocity.1 * 100) % height),
            );

            if final_position.0 < 0 {
                final_position.0 = width + final_position.0;
            }

            if final_position.1 < 0 {
                final_position.1 = height + final_position.1;
            }

            final_position
        })
        .collect();

    [
        (0, width / 2, 0, height / 2),
        (width / 2 + 1, width, 0, height / 2),
        (0, width / 2, height / 2 + 1, height),
        (width / 2 + 1, width, height / 2 + 1, height),
    ]
    .iter()
    .map(|quadrant| {
        (quadrant.2..quadrant.3)
            .map(|y| {
                (quadrant.0..quadrant.1)
                    .map(|x| final_positions.iter().filter(|v| v == &&(x, y)).count() as i32)
                    .sum::<i32>()
            })
            .sum::<i32>()
    })
    .product::<i32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let input = include_str!("./example.txt");
        let result = pt1(&input, (11, 7));
        assert_eq!(result, 12);
    }
}
