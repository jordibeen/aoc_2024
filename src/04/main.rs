use std::time::Instant;

fn main() {
    println!("--- Day 4: Ceres Search ---");

    let input: &str = include_str!("./input.txt");

    let start: Instant = Instant::now();
    println!("pt1: {} (finished in {:.2?})", pt1(&input), start.elapsed());

    let start: Instant = Instant::now();
    println!("pt2: {} (finished in {:.2?})", pt2(&input), start.elapsed());
}

fn pt1(input: &str) -> i32 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let m = grid.len() as i32;
    let n = grid[0].len() as i32;

    let mut ans = 0;
    for i in 0..m {
        for j in 0..n {
            if grid[i as usize][j as usize] == 'X' {
                let directions: Vec<(i32, i32)> = vec![
                    (0, 1),
                    (0, -1),
                    (1, 0),
                    (-1, 0),
                    (1, 1),
                    (-1, -1),
                    (1, -1),
                    (-1, 1),
                ];

                directions.iter().for_each(|(x, y)| {
                    let possibility = (0..4)
                        .filter_map(|c| {
                            if (i + x * c) >= 0
                                && (i + x * c) < m
                                && (j + y * c) >= 0
                                && (j + y * c) < n
                            {
                                Some(grid[(i + x * c) as usize][(j + y * c) as usize])
                            } else {
                                None
                            }
                        })
                        .collect::<String>();

                    if &possibility == "XMAS" {
                        ans += 1;
                    }
                });
            }
        }
    }

    ans
}

fn pt2(input: &str) -> i32 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let m = grid.len() as i32;
    let n = grid[0].len() as i32;

    let mut ans = 0;
    for i in 0..m {
        for j in 0..n {
            if grid[i as usize][j as usize] == 'A' {
                if i - 1 >= 0 && i + 1 < m && j - 1 >= 0 && j + 1 < n {
                    let diag_1 = format!(
                        "{}{}{}",
                        grid[(i - 1) as usize][(j - 1) as usize],
                        grid[i as usize][j as usize],
                        grid[(i + 1) as usize][(j + 1) as usize]
                    );
                    let diag_2 = format!(
                        "{}{}{}",
                        grid[(i + 1) as usize][(j - 1) as usize],
                        grid[i as usize][j as usize],
                        grid[(i - 1) as usize][(j + 1) as usize]
                    );

                    if (diag_1 == "MAS" || diag_1 == "SAM") && (diag_2 == "MAS" || diag_2 == "SAM")
                    {
                        ans += 1;
                    }
                }
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let input = include_str!("./example.txt");
        let result = pt1(&input);
        assert_eq!(result, 18);
    }

    #[test]
    fn pt2_test() {
        let input = include_str!("./example.txt");
        let result = pt2(&input);
        assert_eq!(result, 9);
    }
}
