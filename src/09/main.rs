use std::time::Instant;

fn main() {
    println!("--- Day 9: Disk Fragmenter ---");
    let start: Instant = Instant::now();

    let input: &str = include_str!("./input.txt").trim();
    println!("pt1: {}", pt1(&input));
    println!("Execution time: {:.2?}", start.elapsed());
}

fn pt1(input: &str) -> u64 {
    let mut blocks: Vec<Option<u32>> = input
        .trim()
        .chars()
        .fold((vec![], 0, true), |(mut blocks, mut id, is_file), c| {
            let num = c.to_digit(10).unwrap();

            if is_file {
                (0..num).for_each(|_| {
                    blocks.push(Some(id));
                });
                id += 1;
            } else {
                (0..num).for_each(|_| {
                    blocks.push(None);
                });
            }

            (blocks, id, !is_file)
        })
        .0;

    let mut sorted = blocks.to_vec();
    sorted.iter_mut().enumerate().for_each(|(i, block)| {
        while block.is_none() {
            *block = blocks.pop().unwrap();
        }
        if i >= blocks.len() {
            *block = None;
        }
    });

    sorted
        .iter()
        .enumerate()
        .map(|(id, block)| {
            if let Some(number) = block {
                id as u64 * *number as u64
            } else {
                0_u64
            }
        })
        .sum::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let input = include_str!("./example.txt");
        let result = pt1(&input);
        assert_eq!(result, 1928);
    }
}
