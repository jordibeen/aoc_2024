use std::time::Instant;

fn main() {
    println!("--- Day 15: Warehouse Woes ---");
    let start: Instant = Instant::now();

    let input: &str = include_str!("./input.txt");
    println!("pt1: {}", pt1(&input));
    println!("Execution time: {:.2?}", start.elapsed());
}

fn pt1(input: &str) -> i32 {
    let mut robot_pos: (isize, isize) = (isize::MAX, isize::MAX);
    let (mut grid, moves): (Vec<Vec<char>>, Vec<(isize, isize)>) = input
        .split_once("\n\n")
        .map(|(l, r)| {
            let grid: Vec<Vec<char>> = l
                .lines()
                .enumerate()
                .map(|(y, row)| {
                    row.chars()
                        .enumerate()
                        .map(|(x, c)| {
                            if c == '@' {
                                robot_pos = (y as isize, x as isize)
                            }
                            c
                        })
                        .collect()
                })
                .collect();

            let moves: Vec<(isize, isize)> = r
                .lines()
                .flat_map(|line| {
                    line.chars()
                        .map(|c| match c {
                            '^' => (-1, 0),
                            '>' => (0, 1),
                            'v' => (1, 0),
                            '<' => (0, -1),
                            _ => unreachable!(),
                        })
                        .collect::<Vec<(isize, isize)>>()
                })
                .collect();

            (grid, moves)
        })
        .unwrap();

    moves.iter().for_each(|dir| {
        let next = ((robot_pos.0 + dir.0), (robot_pos.1 + dir.1));
        match grid[next.0 as usize][next.1 as usize] {
            '.' => {
                grid[next.0 as usize][next.1 as usize] = '@';
                grid[robot_pos.0 as usize][robot_pos.1 as usize] = '.';
                robot_pos = next;
            }
            'O' => {
                let mut rocks = vec![next];
                let mut behind = ((next.0 + dir.0), (next.1 + dir.1));
                while grid[behind.0 as usize][behind.1 as usize] == 'O' {
                    rocks.push(behind);
                    behind = ((behind.0 + dir.0), (behind.1 + dir.1));
                }
                if grid[behind.0 as usize][behind.1 as usize] == '.' {
                    grid[behind.0 as usize][behind.1 as usize] = 'O';
                    grid[next.0 as usize][next.1 as usize] = '@';
                    grid[robot_pos.0 as usize][robot_pos.1 as usize] = '.';
                    robot_pos = next;
                }
            }
            '#' => {}
            _ => unreachable!(),
        }
    });

    grid.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(x, c)| {
                    if c == &'O' {
                        Some((100 * y + x) as i32)
                    } else {
                        None
                    }
                })
                .sum::<i32>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test_1() {
        let input = include_str!("./example_1.txt");
        let result = pt1(&input);
        assert_eq!(result, 2028);
    }

    #[test]
    fn pt1_test_2() {
        let input = include_str!("./example_2.txt");
        let result = pt1(&input);
        assert_eq!(result, 10092);
    }
}
