use std::time::Instant;

fn main() {
    println!("--- Day 9: Disk Fragmenter ---");
    let start: Instant = Instant::now();

    let input: &str = include_str!("./input.txt").trim();
    let blocks = parse(&input);
    println!("pt1: {}", pt1(blocks.to_vec()));
    println!("pt2: {}", pt2(blocks.to_vec()));
    println!("Execution time: {:.2?}", start.elapsed());
}

fn parse(input: &str) -> Vec<Option<u32>> {
    input
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
        .0
}

fn pt1(mut blocks: Vec<Option<u32>>) -> u64 {
    let mut sorted = blocks.to_vec();
    sorted.iter_mut().enumerate().for_each(|(i, block)| {
        while block.is_none() {
            *block = blocks.pop().unwrap();
        }
        if i >= blocks.len() {
            *block = None;
        }
    });

    calculate_checksum(&sorted)
}

fn pt2(mut blocks: Vec<Option<u32>>) -> u64 {
    let mut r = blocks.len() - 1;
    while r > 0 {
        let r_block = blocks[r];
        if r_block.is_some() {
            let mut rr = r;
            while blocks[rr] == r_block && rr > 0 {
                rr -= 1;
            }
            let file_size = r - rr;

            let mut l = 0;
            while l < rr {
                if blocks[l].is_none() {
                    let mut ll = l;
                    while blocks[ll].is_none() {
                        ll += 1;
                    }
                    let free_size = ll - l;

                    if file_size <= free_size {
                        blocks[rr + 1..rr + 1 + file_size]
                            .iter_mut()
                            .rev()
                            .for_each(|block| {
                                *block = None;
                            });

                        blocks[l..l + file_size].iter_mut().for_each(|block| {
                            *block = r_block;
                        });

                        break;
                    }
                    r = rr + 1;
                }
                l += 1;
            }
        }
        r -= 1;
    }

    calculate_checksum(&blocks)
}

fn calculate_checksum(blocks: &Vec<Option<u32>>) -> u64 {
    blocks
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
        let blocks = parse(&input);
        let result = pt1(blocks);
        assert_eq!(result, 1928);
    }

    #[test]
    fn pt2_test() {
        let input = include_str!("./example.txt");
        let blocks = parse(&input);
        let result = pt2(blocks);
        assert_eq!(result, 2858);
    }
}
