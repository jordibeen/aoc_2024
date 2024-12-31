use std::collections::HashMap;
use std::time::Instant;

type Position = (isize, isize);
enum Keypad {
    Numeric,
    Directional,
}

fn main() {
    println!("--- Day 21: Keypad Conundrum ---");
    let start: Instant = Instant::now();

    let input: &str = include_str!("./input.txt");
    println!("pt1: {}", pt1(&input));
    println!("Execution time: {:.2?}", start.elapsed());
}

fn pt1(input: &str) -> i32 {
    let numeric_keypad: Vec<Vec<char>> = vec![
        vec!['7', '8', '9'],
        vec!['4', '5', '6'],
        vec!['1', '2', '3'],
        vec!['#', '0', 'A'],
    ];
    let directional_keypad: Vec<Vec<char>> = vec![vec!['#', '^', 'A'], vec!['<', 'v', '>']];

    let num_pad: HashMap<char, Position> =
        numeric_keypad
            .iter()
            .enumerate()
            .fold(HashMap::new(), |mut num_pad, (y, row)| {
                row.iter().enumerate().for_each(|(x, c)| {
                    if c != &'#' {
                        num_pad.insert(*c, (y as isize, x as isize));
                    }
                });
                num_pad
            });
    let dir_pad: HashMap<char, Position> =
        directional_keypad
            .iter()
            .enumerate()
            .fold(HashMap::new(), |mut dir_pad, (y, row)| {
                row.iter().enumerate().for_each(|(x, c)| {
                    if c != &'#' {
                        dir_pad.insert(*c, (y as isize, x as isize));
                    }
                });
                dir_pad
            });

    input
        .lines()
        .map(|line| {
            let mut code = line.chars().collect::<Vec<char>>();
            code = get_code(Keypad::Numeric, &num_pad, &dir_pad, code);

            (0..2).for_each(|_| {
                code = get_code(Keypad::Directional, &num_pad, &dir_pad, code.clone());
            });

            line.trim_end_matches("A").parse::<i32>().unwrap() * code.len() as i32
        })
        .sum::<i32>()
}

fn get_code(
    pad_type: Keypad,
    num_pad: &HashMap<char, Position>,
    dir_pad: &HashMap<char, Position>,
    characters: Vec<char>,
) -> Vec<char> {
    let (initial_pos, keypad) = match pad_type {
        Keypad::Numeric => ((3, 2), num_pad),
        Keypad::Directional => ((0, 2), dir_pad),
    };

    characters
        .iter()
        .fold((vec![], initial_pos), |(mut chars, mut pos), c| {
            let char_pos = keypad.get(&c).unwrap();
            let dist = ((char_pos.0 - pos.0), (char_pos.1 - pos.1));

            let horizontal_moves = match dist.1.cmp(&0) {
                std::cmp::Ordering::Greater => vec!['>'; dist.1 as usize],
                std::cmp::Ordering::Less => vec!['<'; dist.1.abs() as usize],
                std::cmp::Ordering::Equal => vec![],
            };
            let vertical_moves = match dist.0.cmp(&0) {
                std::cmp::Ordering::Greater => vec!['v'; dist.0 as usize],
                std::cmp::Ordering::Less => vec!['^'; dist.0.abs() as usize],
                std::cmp::Ordering::Equal => vec![],
            };

            if dist.0 > 0 && keypad.values().any(|v| v == &(char_pos.0, pos.1)) {
                chars.extend(vertical_moves);
                chars.extend(horizontal_moves);
                chars.push('A');
                pos = *char_pos;
                return (chars, pos);
            }

            if keypad.values().any(|v| v == &(pos.0, char_pos.1)) {
                chars.extend(horizontal_moves);
                chars.extend(vertical_moves);
                chars.push('A');
                pos = *char_pos;
                return (chars, pos);
            }

            if keypad.values().any(|v| v == &(char_pos.0, pos.1)) {
                chars.extend(vertical_moves);
                chars.extend(horizontal_moves);
                chars.push('A');
                pos = *char_pos;
                return (chars, pos);
            }

            (chars, pos)
        })
        .0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let input = include_str!("./example.txt");
        let result = pt1(&input);
        assert_eq!(result, 126384);
    }
}
