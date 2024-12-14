use regex::Regex;

fn main() {
    let input: &str = include_str!("./input.txt");
    let (positions, velocities) = parse(&input);
    pt2(&positions, &velocities);
    println!("--- Day 14: Restroom Redoubt ---");
    println!("pt2: see above :)");
    println!("pt1: {}", pt1(&positions, &velocities, (101, 103)));
}

fn parse(input: &str) -> (Vec<(isize, isize)>, Vec<(isize, isize)>) {
    input
        .lines()
        .fold((vec![], vec![]), |(mut positions, mut velocities), line| {
            let digits: Vec<isize> = Regex::new(r"(-?\d+)")
                .unwrap()
                .captures_iter(line)
                .map(|cap| cap.get(0).unwrap().as_str().parse::<isize>().unwrap())
                .collect();

            positions.push((digits[0], digits[1]));
            velocities.push((digits[2], digits[3]));

            (positions, velocities)
        })
}

fn pt1(
    positions: &Vec<(isize, isize)>,
    velocities: &Vec<(isize, isize)>,
    size: (isize, isize),
) -> i32 {
    let final_positions: Vec<(isize, isize)> = simulate(&positions, &velocities, size, 100);

    [
        (0, size.0 / 2, 0, size.1 / 2),
        (size.0 / 2 + 1, size.0, 0, size.1 / 2),
        (0, size.0 / 2, size.1 / 2 + 1, size.1),
        (size.0 / 2 + 1, size.0, size.1 / 2 + 1, size.1),
    ]
    .iter()
    .map(|quadrant| {
        (quadrant.2..quadrant.3)
            .map(|y| {
                (quadrant.0..quadrant.1)
                    .map(|x| final_positions.iter().filter(|v| v == &&(x, y)).count() as i32)
                    .sum::<i32>()
            })
            .sum::<i32>()
    })
    .product::<i32>()
}

fn pt2(positions: &Vec<(isize, isize)>, velocities: &Vec<(isize, isize)>) {
    let (width, height) = (101_isize, 103_isize);

    // Starting simulation at 8200 to avoid the long wait ;)
    (8200..8271).for_each(|seconds| {
        println!("=========== {} ==========", seconds);
        let final_positions = simulate(&positions, &velocities, (101, 103), seconds);

        (0..height).for_each(|y| {
            (0..width).for_each(|x| {
                if final_positions.contains(&(y, x)) {
                    print!("#")
                } else {
                    print!(".")
                }
            });
            println!()
        });
    });
}

fn simulate(
    positions: &Vec<(isize, isize)>,
    velocities: &Vec<(isize, isize)>,
    size: (isize, isize),
    seconds: i32,
) -> Vec<(isize, isize)> {
    positions
        .iter()
        .enumerate()
        .map(|(i, pos)| {
            let mut new_pos = (
                ((pos.0 + velocities[i].0 * seconds as isize) % size.0),
                ((pos.1 + velocities[i].1 * seconds as isize) % size.1),
            );

            if new_pos.0 < 0 {
                new_pos.0 = size.0 + new_pos.0;
            }

            if new_pos.1 < 0 {
                new_pos.1 = size.1 + new_pos.1;
            }

            new_pos
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let input = include_str!("./example.txt");
        let (positions, velocities) = parse(&input);
        let result = pt1(&positions, &velocities, (11, 7));
        assert_eq!(result, 12);
    }
}
