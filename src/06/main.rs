use std::collections::HashSet;
use std::time::Instant;

const DIRECTIONS: &[(i32, i32)] = &[(-1, 0), (0, 1), (1, 0), (0, -1)];

fn main() {
    println!("--- Day 6: Guard Gallivant ---");
    let input: &str = include_str!("./input.txt");

    let start: Instant = Instant::now();

    let (grid, pos) = parse(&input);
    println!("pt1: {}", pt1(&grid, pos));
    println!("pt2: {}", pt2(&grid, pos));
    println!("Execution time: {:.2?}", start.elapsed());
}

fn parse(input: &str) -> (Vec<Vec<char>>, (i32, i32)) {
    let mut pos: (i32, i32) = (0, 0);
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

    (grid, pos)
}

fn pt1(grid: &Vec<Vec<char>>, starting_pos: (i32, i32)) -> i32 {
    let (visited, _) = walk(grid, starting_pos, None);

    visited.len() as i32
}

fn pt2(grid: &Vec<Vec<char>>, starting_pos: (i32, i32)) -> i32 {
    let (visited, _) = walk(grid, starting_pos, None);

    visited
        .iter()
        .filter(|square| walk(&grid, starting_pos, Some(**square)).1)
        .count() as i32
}

fn walk(
    grid: &Vec<Vec<char>>,
    mut pos: (i32, i32),
    new_obstacle: Option<(i32, i32)>,
) -> (HashSet<(i32, i32)>, bool) {
    let m = grid.len();
    let n = grid[0].len();

    let mut dir_index = 0;
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut visited_with_direction: HashSet<((i32, i32), usize)> = HashSet::new();

    loop {
        visited.insert(pos);

        if visited_with_direction.contains(&(pos, dir_index)) {
            return (visited, true);
        }

        visited_with_direction.insert((pos, dir_index));

        let dir = DIRECTIONS[dir_index];
        let next = ((pos.0 + dir.0), (pos.1 + dir.1));

        if next.0 < 0 || next.0 as usize >= m || next.1 < 0 || next.1 as usize >= n {
            break;
        }

        if grid[next.0 as usize][next.1 as usize] == '#'
            || (new_obstacle.is_some() && new_obstacle.unwrap() == next)
        {
            dir_index = (dir_index + 1) % DIRECTIONS.len();
        } else {
            pos = next;
        }
    }

    (visited, false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let input = include_str!("./example.txt");
        let (grid, pos) = parse(&input);
        let result = pt1(&grid, pos);
        assert_eq!(result, 41);
    }

    #[test]
    fn pt2_test() {
        let input = include_str!("./example.txt");
        let (grid, pos) = parse(&input);
        let result = pt2(&grid, pos);
        assert_eq!(result, 6);
    }
}
