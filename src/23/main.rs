use std::collections::{HashMap, HashSet};
use std::time::Instant;

fn main() {
    println!("--- Day 23: LAN Party ---");
    let start: Instant = Instant::now();

    let input: &str = include_str!("./input.txt");
    println!("pt1: {}", pt1(&input));
    println!("Execution time: {:.2?}", start.elapsed());
}

fn pt1(input: &str) -> i32 {
    let mut connections: HashMap<String, Vec<String>> = HashMap::new();
    input.lines().for_each(|line| {
        let (l, r) = line.split_once("-").unwrap();
        connections
            .entry(l.to_string())
            .and_modify(|v| v.push(r.to_string()))
            .or_insert(vec![r.to_string()]);
        connections
            .entry(r.to_string())
            .and_modify(|v| v.push(l.to_string()))
            .or_insert(vec![l.to_string()]);
    });

    let mut interconnected: HashSet<Vec<String>> = HashSet::new();
    connections.iter().for_each(|(conn_1, conns_1)| {
        conns_1.iter().for_each(|conn_2| {
            if let Some(conns_2) = connections.get(conn_2) {
                conns_2.iter().for_each(|conn_3| {
                    if let Some(conns_3) = connections.get(conn_3) {
                        conns_3.iter().for_each(|conn_4| {
                            if conn_4 == conn_1 {
                                let mut v = vec![
                                    conn_1.to_string(),
                                    conn_2.to_string(),
                                    conn_3.to_string(),
                                ];
                                v.sort();
                                interconnected.insert(v);
                            }
                        });
                    }
                });
            }
        });
    });

    interconnected
        .iter()
        .filter(|computers| computers.iter().any(|computer| computer.starts_with("t")))
        .count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let input = include_str!("./example.txt");
        let result = pt1(&input);
        assert_eq!(result, 7);
    }
}
