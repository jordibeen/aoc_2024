use std::collections::HashSet;
use std::time::Instant;

fn main() {
    println!("--- Day 6: Guard Gallivant ---");
    let start: Instant = Instant::now();

    let input: &str = include_str!("./input.txt");
    println!("pt1: {} (finished in {:.2?})", pt1(&input), start.elapsed());
}

fn pt1(input: &str) -> i32 {
    let mut pos = (0, 0);
    let grid: Vec<Vec<char>> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, char)| {
                    if char == '^' {
                        pos = (y as i32, x as i32);
                    };
                    char
                })
                .collect()
        })
        .collect();

    let m = grid.len();
    let n = grid[0].len();

    let mut dir = (-1, 0);
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    loop {
        visited.insert(pos);

        let next = ((pos.0 + dir.0), (pos.1 + dir.1));

        if next.0 < 0 || next.0 as usize == m || next.1 < 0 || next.1 as usize == n {
            break;
        }

        if grid[next.0 as usize][next.1 as usize] == '#' {
            dir = match dir {
                (-1, 0) => (0, 1),
                (0, 1) => (1, 0),
                (1, 0) => (0, -1),
                (0, -1) => (-1, 0),
                _ => unreachable!(),
            }
        } else {
            pos = next;
        }
    }

    visited.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let input = include_str!("./example.txt");
        let result = pt1(&input);
        assert_eq!(result, 41);
    }
}
