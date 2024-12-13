use std::collections::{HashSet, VecDeque};
use std::time::Instant;

fn main() {
    println!("--- Day 12: Garden Groups ---");
    let start: Instant = Instant::now();

    let input: &str = include_str!("./input.txt");
    let (regions, pt1_ans) = parse(&input);
    println!("pt1: {}", pt1_ans);
    println!("pt2: {}", pt2(regions));
    println!("Execution time: {:.2?}", start.elapsed());
}

fn parse(input: &str) -> (Vec<HashSet<(isize, isize)>>, i32) {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().map(|char| char).collect())
        .collect();

    let m = grid.len();
    let n = grid[0].len();
    let mut visited: HashSet<(isize, isize)> = HashSet::new();

    let mut pt1_ans = 0;
    let regions: Vec<HashSet<(isize, isize)>> = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(x, c)| {
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

                        pt1_ans += region.len() * total_perimeter;
                        Some(region)
                    } else {
                        None
                    }
                })
                .collect::<Vec<HashSet<(isize, isize)>>>()
        })
        .collect();

    (regions, pt1_ans as i32)
}

fn pt2(regions: Vec<HashSet<(isize, isize)>>) -> i32 {
    regions
        .iter()
        .map(|region| {
            let mut sides = 0;
            let mut bounds: (isize, isize, isize, isize) = (0, 0, 0, 0);

            region.iter().for_each(|tile| {
                bounds.0 = bounds.0.min(tile.0);
                bounds.1 = bounds.1.max(tile.0 + 1);
                bounds.2 = bounds.2.min(tile.1);
                bounds.3 = bounds.3.max(tile.1 + 1);
            });

            (bounds.0..bounds.1).for_each(|y| {
                let mut seen: HashSet<(isize, isize)> = HashSet::new();
                (bounds.2..bounds.3).for_each(|x| {
                    let mut i = 0;
                    if region.contains(&((y, x)))
                        && !region.contains(&(y - 1, x))
                        && !seen.contains(&(y, x))
                    {
                        sides += 1;
                        while region.contains(&(y, x + i)) && !region.contains(&(y - 1, x + i)) {
                            i += 1;
                            seen.insert((y, x + i));
                        }
                    }
                });
            });

            (bounds.0..bounds.1).rev().for_each(|y| {
                let mut seen: HashSet<(isize, isize)> = HashSet::new();
                (bounds.2..bounds.3).for_each(|x| {
                    let mut i = 0;
                    if region.contains(&((y, x)))
                        && !region.contains(&(y + 1, x))
                        && !seen.contains(&(y, x))
                    {
                        sides += 1;
                        while region.contains(&(y, x + i)) && !region.contains(&(y + 1, x + i)) {
                            i += 1;
                            seen.insert((y, x + i));
                        }
                    }
                });
            });

            (bounds.2..bounds.3).for_each(|x| {
                let mut seen: HashSet<(isize, isize)> = HashSet::new();
                (bounds.0..bounds.1).for_each(|y| {
                    let mut i = 0;
                    if region.contains(&((y, x)))
                        && !region.contains(&(y, x + 1))
                        && !seen.contains(&(y, x))
                    {
                        sides += 1;
                        while region.contains(&(y + i, x)) && !region.contains(&(y + i, x + 1)) {
                            i += 1;
                            seen.insert((y + i, x));
                        }
                    }
                });
            });

            (bounds.2..bounds.3).rev().for_each(|x| {
                let mut seen: HashSet<(isize, isize)> = HashSet::new();
                (bounds.0..bounds.1).for_each(|y| {
                    let mut i = 0;
                    if region.contains(&((y, x)))
                        && !region.contains(&(y, x - 1))
                        && !seen.contains(&(y, x))
                    {
                        sides += 1;
                        while region.contains(&(y + i, x)) && !region.contains(&(y + i, x - 1)) {
                            i += 1;
                            seen.insert((y + i, x));
                        }
                    }
                });
            });

            sides * region.len() as i32
        })
        .sum::<i32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test1() {
        let input = include_str!("./example_1.txt");
        let (_, pt1_ans) = parse(&input);
        assert_eq!(pt1_ans, 140);
    }

    #[test]
    fn pt1_test2() {
        let input = include_str!("./example_2.txt");
        let (_, pt1_ans) = parse(&input);
        assert_eq!(pt1_ans, 772);
    }

    #[test]
    fn pt1_test3() {
        let input = include_str!("./example_3.txt");
        let (_, pt1_ans) = parse(&input);
        assert_eq!(pt1_ans, 1930);
    }

    #[test]
    fn pt2_test1() {
        let input = include_str!("./example_1.txt");
        let (regions, _) = parse(&input);
        let result = pt2(regions);
        assert_eq!(result, 80);
    }

    #[test]
    fn pt2_test2() {
        let input = include_str!("./example_2.txt");
        let (regions, _) = parse(&input);
        let result = pt2(regions);
        assert_eq!(result, 436);
    }

    #[test]
    fn pt2_test3() {
        let input = include_str!("./example_4.txt");
        let (regions, _) = parse(&input);
        let result = pt2(regions);
        assert_eq!(result, 236);
    }

    #[test]
    fn pt2_test4() {
        let input = include_str!("./example_5.txt");
        let (regions, _) = parse(&input);
        let result = pt2(regions);
        assert_eq!(result, 368);
    }

    #[test]
    fn pt2_test5() {
        let input = include_str!("./example_3.txt");

        let (regions, _) = parse(&input);
        let result = pt2(regions);
        assert_eq!(result, 1206);
    }
}
