use std::collections::HashMap;
use std::time::Instant;

#[derive(Debug)]
struct Connection {
    wire: String,
    in_left: String,
    gate_type: String,
    in_right: String,
}

fn main() {
    println!("--- Day 24: Crossed Wires ---");
    let start: Instant = Instant::now();

    let input: &str = include_str!("./input.txt");
    println!("pt1: {}", pt1(&input));
    println!("Execution time: {:.2?}", start.elapsed());
}

fn pt1(input: &str) -> i64 {
    let mut z_vals: Vec<Option<bool>> = vec![];
    let (mut wire_vals, connections) = input
        .split_once("\n\n")
        .map(|(w, c)| {
            let wire_vals: HashMap<String, bool> =
                w.lines().fold(HashMap::new(), |mut map, line| {
                    let (wire, val) = line
                        .split_once(": ")
                        .map(|(l, r)| (l.to_string(), r == "1"))
                        .unwrap();
                    map.insert(wire, val);
                    map
                });

            let connections: Vec<Connection> = c
                .lines()
                .map(|line| {
                    let s = line.split(" ").collect::<Vec<&str>>();
                    if s[4].starts_with("z") {
                        z_vals.push(None);
                    }
                    Connection {
                        in_left: s[0].to_string(),
                        gate_type: s[1].to_string(),
                        in_right: s[2].to_string(),
                        wire: s[4].to_string(),
                    }
                })
                .collect();

            (wire_vals, connections)
        })
        .unwrap();

    while z_vals.iter().any(|v| v.is_none()) {
        connections.iter().for_each(|connection| {
            let left_val = wire_vals.get(&connection.in_left);
            let right_val = wire_vals.get(&connection.in_right);

            if left_val.is_some() && right_val.is_some() {
                let val = match connection.gate_type.as_str() {
                    "AND" => *left_val.unwrap() && *right_val.unwrap(),
                    "OR" => *left_val.unwrap() || *right_val.unwrap(),
                    "XOR" => *left_val.unwrap() != *right_val.unwrap(),
                    _ => unreachable!(),
                };

                wire_vals.insert(connection.wire.to_string(), val);

                if connection.wire.starts_with("z") {
                    let i = &connection
                        .wire
                        .trim_start_matches("z")
                        .parse::<usize>()
                        .unwrap();
                    z_vals[*i] = Some(val);
                }
            }
        });
    }

    let binary = z_vals
        .iter()
        .rev()
        .map(|z| match z.unwrap() {
            true => "1",
            false => "0",
        })
        .collect::<String>();

    i64::from_str_radix(&binary, 2).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test_1() {
        let input = include_str!("./example_1.txt");
        let result = pt1(&input);
        assert_eq!(result, 4);
    }

    #[test]
    fn pt1_test_2() {
        let input = include_str!("./example_2.txt");
        let result = pt1(&input);
        assert_eq!(result, 2024);
    }
}
