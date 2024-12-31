use std::collections::HashMap;
use std::time::Instant;

#[derive(Hash, Eq, PartialEq, Clone)]
enum Keypad {
    Numeric,
    Directional,
}

type Position = (isize, isize);

fn main() {
    println!("--- Day 21: Keypad Conundrum ---");
    let source: Instant = Instant::now();

    let (num_pad, dir_pad) = create_pads();
    let input: &str = include_str!("./input.txt");
    println!("pt1: {}", pt1(&num_pad, &dir_pad, &input));
    println!("pt2: {}", pt2(&num_pad, &dir_pad, &input));
    println!("Execution time: {:.2?}", source.elapsed());
}

fn create_pads() -> (HashMap<char, Position>, HashMap<char, Position>) {
    let num_pad: HashMap<char, Position> = vec![
        vec!['7', '8', '9'],
        vec!['4', '5', '6'],
        vec!['1', '2', '3'],
        vec!['#', '0', 'A'],
    ]
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

    let dir_pad: HashMap<char, Position> = vec![vec!['#', '^', 'A'], vec!['<', 'v', '>']]
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

    (num_pad, dir_pad)
}

fn pt1(num_pad: &HashMap<char, Position>, dir_pad: &HashMap<char, Position>, input: &str) -> i32 {
    let mut cache = &mut HashMap::new();
    input
        .lines()
        .map(|line| {
            let min_sequence_length = get_min_length(
                &mut cache,
                Keypad::Numeric,
                &num_pad,
                &dir_pad,
                line.chars().collect::<Vec<char>>(),
                0,
                3,
            );

            line.trim_end_matches("A").parse::<i32>().unwrap() * min_sequence_length as i32
        })
        .sum()
}

fn pt2(num_pad: &HashMap<char, Position>, dir_pad: &HashMap<char, Position>, input: &str) -> i64 {
    let mut cache = &mut HashMap::new();
    input
        .lines()
        .map(|line| {
            let min_sequence_length = get_min_length(
                &mut cache,
                Keypad::Numeric,
                &num_pad,
                &dir_pad,
                line.chars().collect::<Vec<char>>(),
                0,
                26,
            );

            line.trim_end_matches("A").parse::<i64>().unwrap() * min_sequence_length as i64
        })
        .sum()
}

fn get_min_length(
    cache: &mut HashMap<(Keypad, Vec<char>, u8), i64>,
    pad_type: Keypad,
    num_pad: &HashMap<char, Position>,
    dir_pad: &HashMap<char, Position>,
    code: Vec<char>,
    depth: u8,
    max_depth: u8,
) -> i64 {
    if let Some(cached_val) = cache.get(&(pad_type.clone(), code.clone(), depth)) {
        return *cached_val;
    }

    if depth == max_depth {
        return code.len() as i64;
    }

    let key_pad = match pad_type {
        Keypad::Numeric => num_pad,
        Keypad::Directional => dir_pad,
    };

    let mut prev = 'A';
    let mut min_length = 0;
    for c in &code {
        min_length += get_min_length(
            cache,
            Keypad::Directional,
            num_pad,
            dir_pad,
            get_sequence(key_pad, prev, *c),
            depth + 1,
            max_depth,
        );
        prev = *c;
    }

    cache.insert((pad_type.clone(), code.clone(), depth), min_length);

    min_length
}

fn get_sequence(key_pad: &HashMap<char, Position>, source: char, target: char) -> Vec<char> {
    let mut sequence = vec![];

    let source_pos = key_pad.get(&source).unwrap();
    let target_pos = key_pad.get(&target).unwrap();
    let dist = ((target_pos.0 - source_pos.0), (target_pos.1 - source_pos.1));

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

    if dist.1 > 0 && key_pad.values().any(|v| v == &(target_pos.0, source_pos.1)) {
        sequence.extend(vertical_moves);
        sequence.extend(horizontal_moves);
        sequence.push('A');
        return sequence;
    }

    if key_pad.values().any(|v| v == &(source_pos.0, target_pos.1)) {
        sequence.extend(horizontal_moves);
        sequence.extend(vertical_moves);
        sequence.push('A');
        return sequence;
    }

    if key_pad.values().any(|v| v == &(target_pos.0, source_pos.1)) {
        sequence.extend(vertical_moves);
        sequence.extend(horizontal_moves);
        sequence.push('A');
        return sequence;
    }

    sequence
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let input = include_str!("./example.txt");
        let (num_pad, dir_pad) = create_pads();
        let result = pt1(&num_pad, &dir_pad, &input);
        assert_eq!(result, 126384);
    }
}
