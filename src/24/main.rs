use std::collections::{HashMap, VecDeque};
use std::time::Instant;

#[derive(Clone)]
pub struct Connection {
    left: String,
    operation: String,
    right: String,
}

fn main() {
    println!("--- Day 24: Crossed Wires ---");
    let start: Instant = Instant::now();

    let input: &str = include_str!("./input.txt");
    let (wire_vals, connections) = parse(&input);
    println!("pt1: {}", pt1(wire_vals.clone(), connections.clone()));
    println!(
        "pt2: {}",
        pt2(wire_vals.clone(), connections.clone()).expect("Part 2 needs some attention..")
    );
    println!("Execution time: {:.2?}", start.elapsed());
}

fn parse(input: &str) -> (HashMap<String, bool>, HashMap<String, Connection>) {
    input
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

            let connections: HashMap<String, Connection> =
                c.lines().fold(HashMap::new(), |mut map, line| {
                    let s = line.split(" ").collect::<Vec<&str>>();
                    map.insert(
                        s[4].to_string(),
                        Connection {
                            left: s[0].to_string(),
                            operation: s[1].to_string(),
                            right: s[2].to_string(),
                        },
                    );
                    map
                });

            (wire_vals, connections)
        })
        .unwrap()
}

fn pt1(wire_vals: HashMap<String, bool>, connections: HashMap<String, Connection>) -> i64 {
    let wire_vals = test_circuit(wire_vals, connections, None, None);
    get_val(&wire_vals, "z")
}

fn pt2(
    wire_vals: HashMap<String, bool>,
    mut connections: HashMap<String, Connection>,
) -> Option<String> {
    let swaps: Vec<(String, String)> = vec![
        ("z10".to_string(), "ggn".to_string()),
        ("ndw".to_string(), "jcb".to_string()),
        ("z32".to_string(), "grm".to_string()),
        ("z39".to_string(), "twr".to_string()),
    ];

    swaps.iter().for_each(|(l, r)| {
        let l_conn = connections.remove(l).unwrap().to_owned();
        let r_conn = connections.remove(r).unwrap().to_owned();
        connections.insert(l.to_string(), r_conn);
        connections.insert(r.to_string(), l_conn);
    });

    // Figure out which bits caused faulty behaviour.
    let faulty_bits: Vec<usize> = (0..45)
        .filter_map(|i| {
            let x = 1 << 0;
            let y = 1 << i;
            let wire_vals = test_circuit(wire_vals.clone(), connections.clone(), Some(x), Some(y));
            if (x + y) != get_val(&wire_vals, "z") {
                Some(i)
            } else {
                None
            }
        })
        .collect();

    // Pretty print z values and their connection up until a certain depth in order to analyse which piece of wiring looks off manually
    if !faulty_bits.is_empty() {
        (0..46).for_each(|i| {
            let wire = format!("z{:0>2}", i);
            println!("--- wire {wire} ----");
            pprint(&connections, wire, 4, 0);
        });

        println!(
            "Faulty bits {:?} need some attention.. Please analyse their wiring in the output above.",
            faulty_bits
        );

        return None;
    }

    let mut ans: Vec<String> = swaps
        .into_iter()
        .map(|(l, r)| vec![l, r])
        .flatten()
        .collect();
    ans.sort();

    Some(ans.join(","))
}

pub fn get_val(wire_vals: &HashMap<String, bool>, gate_type: &str) -> i64 {
    let mut gate_vals: Vec<(String, bool)> = wire_vals
        .to_owned()
        .into_iter()
        .filter(|wire_val| wire_val.0.starts_with(gate_type))
        .collect();

    gate_vals.sort_by(|l, r| {
        l.0.trim_start_matches(gate_type)
            .parse::<u8>()
            .unwrap()
            .cmp(&(r.0.trim_start_matches(gate_type).parse::<u8>().unwrap()))
    });

    gate_vals.iter().enumerate().fold(
        0,
        |acc, (i, (_, val))| if *val { acc | 1 << i } else { acc },
    )
}

pub fn test_circuit(
    wire_vals: HashMap<String, bool>,
    connections: HashMap<String, Connection>,
    x_val: Option<i64>,
    y_val: Option<i64>,
) -> HashMap<String, bool> {
    let mut wire_vals_local = wire_vals.clone();

    if let Some(x) = x_val {
        wire_vals_local.iter_mut().for_each(|(wire, val)| {
            if wire.starts_with("x") {
                let bitidx = wire.trim_start_matches("x").parse::<usize>().unwrap();
                *val = ((x >> bitidx) & 1) != 0
            }
        });
    }

    if let Some(y) = y_val {
        wire_vals_local.iter_mut().for_each(|(wire, val)| {
            if wire.starts_with("y") {
                let bitidx = wire.trim_start_matches("y").parse::<usize>().unwrap();
                *val = ((y >> bitidx) & 1) != 0;
            }
        });
    }

    let mut queue: VecDeque<(&String, &Connection)> = VecDeque::from_iter(connections.iter());
    while let Some((wire, connection)) = queue.pop_front() {
        if wire_vals_local.get(wire).is_none() {
            let left_val = wire_vals_local.get(&connection.left);
            let right_val = wire_vals_local.get(&connection.right);
            if left_val.is_none() || right_val.is_none() {
                queue.push_back((
                    &connection.right,
                    connections.get(&connection.right).unwrap(),
                ));
                queue.push_back((&connection.left, connections.get(&connection.left).unwrap()));
                queue.push_back((wire, connection));
            }

            if left_val.is_some() && right_val.is_some() {
                let val = match connection.operation.as_str() {
                    "AND" => *left_val.unwrap() & *right_val.unwrap(),
                    "OR" => *left_val.unwrap() | *right_val.unwrap(),
                    "XOR" => *left_val.unwrap() ^ *right_val.unwrap(),
                    _ => unreachable!(),
                };

                wire_vals_local.insert(wire.to_string(), val);
            }
        }
    }

    wire_vals_local
}

fn pprint(
    connections: &HashMap<String, Connection>,
    wire: String,
    max_depth: usize,
    depth: usize,
) -> () {
    if depth > max_depth {
        return;
    };

    if let Some(connection) = connections.get(&wire) {
        println!(
            "{}{} = {} {} {}",
            String::from_utf8(vec![b' '; depth]).unwrap(),
            wire,
            connection.operation,
            connection.left,
            connection.right
        );
        pprint(
            connections,
            connection.left.to_string(),
            max_depth,
            depth + 1,
        );
        pprint(
            connections,
            connection.right.to_string(),
            max_depth,
            depth + 1,
        );
    } else {
        println!("{}{}", String::from_utf8(vec![b' '; depth]).unwrap(), wire);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test_1() {
        let input = include_str!("./example_1.txt");
        let (wire_vals, connections) = parse(&input);
        let result = pt1(wire_vals, connections);
        assert_eq!(result, 4);
    }

    #[test]
    fn pt1_test_2() {
        let input = include_str!("./example_2.txt");
        let (wire_vals, connections) = parse(&input);
        let result = pt1(wire_vals, connections);
        assert_eq!(result, 2024);
    }
}
