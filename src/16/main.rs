use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::time::Instant;

type Position = (isize, isize);
type Direction = (isize, isize);
const DIRECTIONS: &[Direction] = &[(-1, 0), (0, 1), (0, -1), (1, 0)];

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
    let timer: Instant = Instant::now();

    let input: &str = include_str!("./input.txt");
    let (grid, start, end) = parse(input);
    println!("pt1: {}", pt1(&grid, start, end));
    println!("pt2: {}", pt2(grid, start, end));
    println!("Execution time: {:.2?}", timer.elapsed());
}

fn parse(input: &str) -> (Vec<Vec<char>>, Position, Position) {
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

    (grid, start, end)
}

fn pt1(grid: &Vec<Vec<char>>, start: Position, end: Position) -> i32 {
    let scores = dijkstra(grid, start, (0, 1));
    DIRECTIONS.iter().fold(i32::MAX, |ans, d| {
        if let Some(score) = scores.get(&(end, *d)) {
            ans.min(*score)
        } else {
            ans
        }
    })
}

fn pt2(grid: Vec<Vec<char>>, start: Position, end: Position) -> i32 {
    let scores = dijkstra(&grid, start, (0, 1));

    let mut end_dir = (0, 0);
    let lowest_score = DIRECTIONS.iter().fold(i32::MAX, |mut lowest, d| {
        if let Some(score) = scores.get(&(end, *d)) {
            if *score < lowest {
                lowest = *score;
                end_dir = *d;
            }
        }
        lowest
    });

    let scores_backwards = dijkstra(&grid, end, (-end_dir.0, -end_dir.1));

    let mut tiles: HashSet<Position> = HashSet::new();
    (0..grid.len()).for_each(|y| {
        (0..grid[0].len()).for_each(|x| {
            DIRECTIONS.iter().enumerate().for_each(|(d_i, d)| {
                if let Some(score) = scores.get(&((y as isize, x as isize), *d)) {
                    if let Some(score_backwards) = scores_backwards.get(&(
                        (y as isize, x as isize),
                        *DIRECTIONS.iter().rev().nth(d_i).unwrap(),
                    )) {
                        if score + score_backwards == lowest_score {
                            tiles.insert((y as isize, x as isize));
                        }
                    }
                }
            });
        });
    });

    tiles.len() as i32
}

fn dijkstra(
    grid: &Vec<Vec<char>>,
    start: Position,
    direction: Direction,
) -> HashMap<(Position, Direction), i32> {
    let mut heap: BinaryHeap<Current> = BinaryHeap::from([Current {
        position: start,
        direction,
        score: 0,
    }]);
    let mut scores: HashMap<(Position, Direction), i32> = HashMap::from([((start, direction), 0)]);

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

    scores
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test_1() {
        let input = include_str!("./example_1.txt");
        let (grid, start, end) = parse(input);
        let result = pt1(&grid, start, end);
        assert_eq!(result, 7036);
    }

    #[test]
    fn pt1_test_2() {
        let input = include_str!("./example_2.txt");
        let (grid, start, end) = parse(input);
        let result = pt1(&grid, start, end);
        assert_eq!(result, 11048);
    }

    #[test]
    fn pt2_test_1() {
        let input = include_str!("./example_1.txt");
        let (grid, start, end) = parse(input);
        let result = pt2(grid, start, end);
        assert_eq!(result, 45);
    }

    #[test]
    fn pt2_test_2() {
        let input = include_str!("./example_2.txt");
        let (grid, start, end) = parse(input);
        let result = pt2(grid, start, end);
        assert_eq!(result, 64);
    }
}
