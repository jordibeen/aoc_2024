use std::collections::{HashMap, HashSet};
use std::time::Instant;

fn main() {
    println!("--- Day 8: Resonant Collinearity ---");
    let start: Instant = Instant::now();

    let input: &str = include_str!("./input.txt");
    println!("pt1: {} (finished in {:.2?})", pt1(&input), start.elapsed());
}

fn pt1(input: &str) -> i32 {
    let mut antennas: HashMap<char, Vec<(isize, isize)>> = HashMap::new();
    let grid: Vec<Vec<char>> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, char)| {
                    if char != '.' {
                        antennas
                            .entry(char)
                            .and_modify(|v| v.push((y as isize, x as isize)))
                            .or_insert(vec![(y as isize, x as isize)]);
                    }
                    char
                })
                .collect()
        })
        .collect();

    let m = grid.len();
    let n = grid[0].len();

    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();
    antennas.values().for_each(|locations| {
        locations.iter().for_each(|antenna| {
            locations.iter().for_each(|other| {
                if antenna != other {
                    let dist = (antenna.0 - other.0, antenna.1 - other.1);
                    let antinode = ((antenna.0 + dist.0), (antenna.1 + dist.1));

                    if antinode.0 >= 0
                        && antinode.0 < m as isize
                        && antinode.1 >= 0
                        && antinode.1 < n as isize
                    {
                        antinodes.insert(antinode);
                    }
                }
            });
        });
    });

    antinodes.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let input = include_str!("./example_pt1.txt");
        let result = pt1(&input);
        assert_eq!(result, 14);
    }
}
