use std::collections::{HashMap, HashSet};
use std::time::Instant;

fn main() {
    println!("--- Day 8: Resonant Collinearity ---");
    let start: Instant = Instant::now();

    let input: &str = include_str!("./input.txt");
    let (bounds, antennas) = parse(&input);
    println!("pt1: {}", pt1(&bounds, &antennas));
    println!("pt2: {}", pt2(&bounds, &antennas));
    println!("Execution time: {:.2?}", start.elapsed());
}

fn parse(input: &str) -> ((usize, usize), HashMap<char, Vec<(isize, isize)>>) {
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

    let bounds = (grid.len(), grid[0].len());

    (bounds, antennas)
}

fn pt1(bounds: &(usize, usize), antennas: &HashMap<char, Vec<(isize, isize)>>) -> i32 {
    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();
    antennas.values().for_each(|locations| {
        locations.iter().for_each(|antenna| {
            locations.iter().for_each(|other| {
                if antenna != other {
                    let dist = (antenna.0 - other.0, antenna.1 - other.1);
                    let antinode = ((antenna.0 + dist.0), (antenna.1 + dist.1));

                    if antinode.0 >= 0
                        && antinode.0 < bounds.0 as isize
                        && antinode.1 >= 0
                        && antinode.1 < bounds.1 as isize
                    {
                        antinodes.insert(antinode);
                    }
                }
            });
        });
    });

    antinodes.len() as i32
}

fn pt2(bounds: &(usize, usize), antennas: &HashMap<char, Vec<(isize, isize)>>) -> i32 {
    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();
    antennas.values().for_each(|locations| {
        locations.iter().for_each(|antenna| {
            locations.iter().for_each(|other| {
                if antenna != other {
                    let dist = (antenna.0 - other.0, antenna.1 - other.1);

                    let mut antinode_1 = ((antenna.0 + dist.0), (antenna.1 + dist.1));
                    while antinode_1.0 >= 0
                        && antinode_1.0 < bounds.0 as isize
                        && antinode_1.1 >= 0
                        && antinode_1.1 < bounds.1 as isize
                    {
                        antinodes.insert(antinode_1);
                        antinode_1 = ((antinode_1.0 + dist.0), (antinode_1.1 + dist.1));
                    }

                    let mut antinode_2 = ((antenna.0 - dist.0), (antenna.1 - dist.1));
                    while antinode_2.0 >= 0
                        && antinode_2.0 < bounds.0 as isize
                        && antinode_2.1 >= 0
                        && antinode_2.1 < bounds.1 as isize
                    {
                        antinodes.insert(antinode_2);
                        antinode_2 = ((antinode_2.0 + dist.0), (antinode_2.1 + dist.1));
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
        let input = include_str!("./example.txt");
        let (bounds, antennas) = parse(&input);
        let result = pt1(&bounds, &antennas);
        assert_eq!(result, 14);
    }

    #[test]
    fn pt2_test() {
        let input = include_str!("./example.txt");
        let (bounds, antennas) = parse(&input);
        let result = pt2(&bounds, &antennas);
        assert_eq!(result, 34);
    }
}
