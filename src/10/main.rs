use std::collections::VecDeque;
use std::time::Instant;

const DIRECTIONS: &[(isize, isize)] = &[(-1, 0), (1, 0), (0, -1), (0, 1)];

fn main() {
    println!("--- Day 10: Hoof It ---");
    let start: Instant = Instant::now();

    let input: &str = include_str!("./input.txt");
    println!("pt1: {}", pt1(&input));
    println!("Execution time: {:.2?}", start.elapsed());
}

fn pt1(input: &str) -> i32 {
    let mut trailheads: Vec<(isize, isize)> = vec![];
    let grid: Vec<Vec<i32>> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, char)| {
                    if char == '0' {
                        trailheads.push((y as isize, x as isize));
                    };
                    char.to_digit(10).unwrap() as i32
                })
                .collect()
        })
        .collect();

    let m = grid.len();
    let n = grid[0].len();

    trailheads
        .into_iter()
        .map(|trailhead| {
            let mut queue: VecDeque<((isize, isize), Vec<(isize, isize)>, i32)> =
                VecDeque::from([(trailhead, vec![trailhead], 0)]);

            let mut score = 0;
            let mut ends: Vec<(isize, isize)> = Vec::new();
            while let Some((curr, trail, value)) = queue.pop_front() {
                if value == 9 && !ends.contains(&curr) {
                    score += 1;
                    ends.push(curr);
                }
                DIRECTIONS.iter().for_each(|dir| {
                    let next = ((curr.0 + dir.0), (curr.1 + dir.1));
                    if next.0 >= 0 && next.0 < m as isize && next.1 >= 0 && next.1 < n as isize {
                        if grid[next.0 as usize][next.1 as usize] == value + 1 {
                            let mut trail = trail.clone();
                            trail.push(next);
                            queue.push_front((next, trail, value + 1));
                        }
                    }
                });
            }

            score
        })
        .sum::<i32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let input = include_str!("./example.txt");
        let result = pt1(&input);
        assert_eq!(result, 36);
    }
}
