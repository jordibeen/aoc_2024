use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::time::Instant;

type Position = (isize, isize);
type Direction = (isize, isize);
const DIRECTIONS: &[Direction] = &[(-1, 0), (0, 1), (1, 0), (0, -1)];

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Current {
    position: Position,
    direction: Direction,
    score: i32,
}

impl Ord for Current {
    fn cmp(&self, other: &Self) -> Ordering {
        other.score.cmp(&self.score)
    }
}

impl PartialOrd for Current {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    println!("--- Day 16: Reindeer Maze ---");
    let start: Instant = Instant::now();

    let input: &str = include_str!("./input.txt");
    println!("pt1: {}", pt1(&input));
    println!("Execution time: {:.2?}", start.elapsed());
}

fn pt1(input: &str) -> i32 {
    let (mut start, mut end): (Position, Position) = ((0, 0), (0, 0));
    let grid: Vec<Vec<char>> = input
        .lines()
        .enumerate()
        .map(|(y, row)| {
            row.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == 'S' {
                        start = (y as isize, x as isize)
                    }
                    if c == 'E' {
                        end = (y as isize, x as isize)
                    }
                    c
                })
                .collect()
        })
        .collect();

    let initial_direction = (0, 1);
    let mut heap: BinaryHeap<Current> = BinaryHeap::from([Current {
        score: 0,
        position: start,
        direction: initial_direction,
    }]);
    let mut scores: HashMap<(Position, Direction), i32> =
        HashMap::from([((start, initial_direction), 0)]);

    while let Some(curr) = heap.pop() {
        let next: Position = (
            curr.position.0 + curr.direction.0,
            curr.position.1 + curr.direction.1,
        );

        if grid[next.0 as usize][next.1 as usize] != '#' {
            let adj: (Position, Direction) = (next, curr.direction);
            if scores.get(&adj).is_none() {
                scores.insert(adj, curr.score + 1);
                heap.push(Current {
                    score: curr.score + 1,
                    position: next,
                    direction: curr.direction,
                });
            }
        }

        DIRECTIONS.iter().for_each(|d| {
            let adj: (Position, Direction) = (curr.position, *d);
            if scores.get(&adj).is_none() {
                scores.insert(adj, curr.score + 1000);
                heap.push(Current {
                    score: curr.score + 1000,
                    position: curr.position,
                    direction: *d,
                });
            }
        });
    }

    DIRECTIONS.iter().fold(i32::MAX, |ans, d| {
        if let Some(score) = scores.get(&(end, *d)) {
            ans.min(*score)
        } else {
            ans
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test_1() {
        let input = include_str!("./example_1.txt");
        let result = pt1(&input);
        assert_eq!(result, 7036);
    }

    #[test]
    fn pt1_test_2() {
        let input = include_str!("./example_2.txt");
        let result = pt1(&input);
        assert_eq!(result, 11048);
    }
}
