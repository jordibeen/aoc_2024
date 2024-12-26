use std::collections::{HashMap, HashSet, VecDeque};
use std::time::Instant;

type Position = (isize, isize);
type Direction = (isize, isize);
const DIRECTIONS: &[Direction] = &[(-1, 0), (0, 1), (0, -1), (1, 0)];

fn main() {
    println!("--- Day 20: Race Condition ---");
    let start: Instant = Instant::now();

    let input: &str = include_str!("./input.txt");
    let shortest_path = parse(&input);
    println!("pt1: {}", pt1(&shortest_path, 100));
    println!("pt2: {}", pt2(&shortest_path, 100));
    println!("Execution time: {:.2?}", start.elapsed());
}
fn parse(input: &str) -> HashMap<Position, i32> {
    let (mut start, mut end): (Position, Position) = ((0, 0), (0, 0));
    let size = input.lines().count() as isize;
    let walls: Vec<Position> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(x, c)| {
                    match c {
                        'S' => start = (y as isize, x as isize),
                        'E' => end = (y as isize, x as isize),
                        '#' => {
                            if y != 0 && x != 0 && y != size as usize && x != size as usize {
                                return Some((y as isize, x as isize));
                            }
                        }
                        _ => {}
                    };

                    None
                })
                .collect::<Vec<Position>>()
        })
        .collect();

    let mut queue: VecDeque<(Position, Vec<Position>)> = VecDeque::from([(start, vec![start])]);
    let mut shortest_path: HashMap<Position, i32> = HashMap::new();
    let mut visited: HashSet<Position> = HashSet::new();
    while let Some((position, mut path)) = queue.pop_front() {
        visited.insert(position);

        if position == end {
            path.iter().enumerate().for_each(|(i, p)| {
                shortest_path.insert(*p, i as i32);
            });
            break;
        }

        DIRECTIONS.iter().for_each(|d| {
            let next: Position = (position.0 + d.0, position.1 + d.1);
            if 0 < next.0
                && next.0 < size
                && 0 < next.1
                && next.1 < size
                && !visited.contains(&next)
                && !walls.contains(&next)
            {
                path.push(next);
                queue.push_front((next, path.clone()));
            }
        });
    }

    shortest_path
}

fn pt1(shortest_path: &HashMap<Position, i32>, seconds_to_save: i32) -> i32 {
    shortest_path
        .iter()
        .map(|(pos, steps)| {
            DIRECTIONS
                .iter()
                .filter(|d| {
                    let cheat_pos = (pos.0 + (d.0 * 2), pos.1 + (d.1 * 2));
                    shortest_path
                        .get(&cheat_pos)
                        .is_some_and(|v| (v - *steps - 2) >= seconds_to_save)
                })
                .count() as i32
        })
        .sum()
}

fn pt2(shortest_path: &HashMap<Position, i32>, seconds_to_save: i32) -> i32 {
    shortest_path
        .iter()
        .map(|(l_pos, l_steps)| {
            shortest_path
                .iter()
                .filter(|(r_pos, r_steps)| {
                    let dist = l_pos.0.abs_diff(r_pos.0) + l_pos.1.abs_diff(r_pos.1);
                    dist <= 20 && (*r_steps - l_steps - dist as i32) >= seconds_to_save
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
        let shortest_path = parse(&input);
        let result = pt1(&shortest_path, 2);
        assert_eq!(result, 44);
    }

    #[test]
    fn pt2_test() {
        let input = include_str!("./example.txt");
        let shortest_path = parse(&input);
        let result = pt2(&shortest_path, 50);
        assert_eq!(result, 285);
    }
}
