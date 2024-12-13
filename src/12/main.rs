use std::collections::{HashSet, VecDeque};
use std::time::Instant;

fn main() {
    println!("--- Day 12: Garden Groups ---");
    let start: Instant = Instant::now();

    let input: &str = include_str!("./input.txt");
    println!("pt1: {}", pt1(&input));
    println!("Execution time: {:.2?}", start.elapsed());
}

fn pt1(input: &str) -> i32 {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().map(|char| char).collect())
        .collect();

    let m = grid.len();
    let n = grid[0].len();
    let mut visited: HashSet<(isize, isize)> = HashSet::new();

    let mut ans = 0;
    grid.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, c)| {
            if !visited.contains(&(y as isize, x as isize)) {
                let mut region: HashSet<(isize, isize)> = HashSet::new();
                let mut total_perimeter = 0;
                let mut queue: VecDeque<(isize, isize)> =
                    VecDeque::from([(y as isize, x as isize)]);

                while let Some(pos) = queue.pop_front() {
                    if region.contains(&pos) {
                        continue;
                    }

                    total_perimeter += 4;
                    region.insert(pos);
                    visited.insert(pos);

                    [(-1, 0), (0, 1), (1, 0), (0, -1)].iter().for_each(|dir| {
                        let adj = (pos.0 + dir.0, pos.1 + dir.1);
                        if adj.0 >= 0
                            && adj.0 < m as isize
                            && adj.1 >= 0
                            && adj.1 < n as isize
                            && grid[adj.0 as usize][adj.1 as usize] == *c
                        {
                            total_perimeter -= 1;
                            queue.push_back(adj);
                        }
                    });
                }

                ans += region.len() * total_perimeter;
            }
        });
    });

    ans as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test_1() {
        let input = include_str!("./example_1.txt");
        let result = pt1(&input);
        assert_eq!(result, 140);
    }

    #[test]
    fn pt1_test2() {
        let input = include_str!("./example_2.txt");
        let result = pt1(&input);
        assert_eq!(result, 772);
    }

    #[test]
    fn pt1_test3() {
        let input = include_str!("./example_3.txt");
        let result = pt1(&input);
        assert_eq!(result, 1930);
    }
}
