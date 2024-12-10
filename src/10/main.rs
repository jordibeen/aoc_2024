use std::collections::VecDeque;
use std::time::Instant;

const DIRECTIONS: &[(isize, isize)] = &[(-1, 0), (1, 0), (0, -1), (0, 1)];

fn main() {
    println!("--- Day 10: Hoof It ---");
    let start: Instant = Instant::now();

    let input: &str = include_str!("./input.txt");
    let (grid, trailheads) = parse(&input);
    let (pt1, pt2) = pathfind(grid, trailheads);
    println!("pt1: {}", pt1);
    println!("pt2: {}", pt2);
    println!("Execution time: {:.2?}", start.elapsed());
}

fn parse(input: &str) -> (Vec<Vec<i32>>, Vec<(isize, isize)>) {
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

    (grid, trailheads)
}

fn pathfind(grid: Vec<Vec<i32>>, trailheads: Vec<(isize, isize)>) -> (i32, i32) {
    let m = grid.len();
    let n = grid[0].len();

    let mut pt1 = 0;
    let mut pt2 = 0;
    trailheads.into_iter().for_each(|trailhead| {
        let mut queue: VecDeque<((isize, isize), i32)> = VecDeque::from([(trailhead, 0)]);
        let mut trail_ends: Vec<(isize, isize)> = Vec::new();

        while let Some((curr, value)) = queue.pop_front() {
            if value == 9 {
                if !trail_ends.contains(&curr) {
                    pt1 += 1;
                }
                trail_ends.push(curr);
                pt2 += 1;
            }
            DIRECTIONS.iter().for_each(|dir| {
                let next = ((curr.0 + dir.0), (curr.1 + dir.1));
                if next.0 >= 0 && next.0 < m as isize && next.1 >= 0 && next.1 < n as isize {
                    if grid[next.0 as usize][next.1 as usize] == value + 1 {
                        queue.push_front((next, value + 1));
                    }
                }
            });
        }
    });

    (pt1, pt2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let input = include_str!("./example.txt");
        let (grid, trailheads) = parse(&input);
        let (result, _) = pathfind(grid, trailheads);
        assert_eq!(result, 36);
    }

    #[test]
    fn pt2_test() {
        let input = include_str!("./example.txt");
        let (grid, trailheads) = parse(&input);
        let (_, result) = pathfind(grid, trailheads);
        assert_eq!(result, 81);
    }
}
