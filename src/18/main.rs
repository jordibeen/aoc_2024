use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::time::Instant;

type Position = (isize, isize);
type Direction = (isize, isize);
const DIRECTIONS: &[Direction] = &[(-1, 0), (0, 1), (0, -1), (1, 0)];

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Current {
    position: Position,
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
    println!("--- Day 18: RAM Run ---");
    let start: Instant = Instant::now();

    let input: &str = include_str!("./input.txt");
    let bytes = parse(input);
    println!("pt1: {}", pt1(&bytes, 70, 1024));
    println!("pt1: {}", pt2(bytes, 70));
    println!("Execution time: {:.2?}", start.elapsed());
}

fn parse(input: &str) -> Vec<Position> {
    input
        .lines()
        .map(|row| {
            row.split_once(",")
                .map(|(x, y)| (x.parse::<isize>().unwrap(), y.parse::<isize>().unwrap()))
                .unwrap()
        })
        .collect()
}

fn pt1(bytes: &Vec<Position>, size: isize, byte_amount: usize) -> i32 {
    let corrupted: Vec<Position> = bytes[0..byte_amount].iter().copied().collect();
    let (start, end): (Position, Position) = ((0, 0), (size, size));
    let steps = dijkstra(&corrupted, size, start);

    *steps.get(&end).unwrap()
}

fn pt2(mut bytes: Vec<Position>, size: isize) -> String {
    let (start, end): (Position, Position) = ((0, 0), (size, size));

    let mut blocking_byte;
    loop {
        blocking_byte = bytes.pop();
        let steps = dijkstra(&bytes, size, start);
        if steps.get(&end).is_some() {
            break;
        }
    }

    format!("{},{}", blocking_byte.unwrap().0, blocking_byte.unwrap().1)
}

fn dijkstra(corrupted: &Vec<Position>, size: isize, start: Position) -> HashMap<Position, i32> {
    let mut heap: BinaryHeap<Current> = BinaryHeap::from([Current {
        position: start,
        score: 0,
    }]);
    let mut steps: HashMap<Position, i32> = HashMap::from([(start, 0)]);

    while let Some(curr) = heap.pop() {
        DIRECTIONS.iter().for_each(|d| {
            let next: Position = (curr.position.0 + d.0, curr.position.1 + d.1);
            if 0 <= next.0
                && next.0 <= size
                && 0 <= next.1
                && next.1 <= size
                && !corrupted.contains(&next)
                && (steps.get(&next).is_none()
                    || steps.get(&next).is_some_and(|v| v > &(curr.score + 1)))
            {
                steps.insert(next, curr.score + 1);
                heap.push(Current {
                    score: curr.score + 1,
                    position: next,
                });
            }
        });
    }

    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let input = include_str!("./example.txt");
        let bytes = parse(input);
        let result = pt1(&bytes, 6, 12);
        assert_eq!(result, 22);
    }

    #[test]
    fn pt2_test() {
        let input = include_str!("./example.txt");
        let bytes = parse(input);
        let result = pt2(bytes, 6);
        assert_eq!(result, "6,1".to_string());
    }
}
