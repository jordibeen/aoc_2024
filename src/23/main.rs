use std::collections::{HashMap, HashSet, VecDeque};
use std::time::Instant;

fn main() {
    println!("--- Day 23: LAN Party ---");
    let start: Instant = Instant::now();

    let input: &str = include_str!("./input.txt");
    let connections = parse(&input);
    println!("pt1: {}", pt1(&connections));
    println!("pt2: {}", pt2(&connections));
    println!("Execution time: {:.2?}", start.elapsed());
}
fn parse(input: &str) -> HashMap<String, Vec<String>> {
    input.lines().fold(HashMap::new(), |mut connections, line| {
        let (l, r) = line.split_once("-").unwrap();
        connections
            .entry(l.to_string())
            .and_modify(|v| v.push(r.to_string()))
            .or_insert(vec![r.to_string()]);
        connections
            .entry(r.to_string())
            .and_modify(|v| v.push(l.to_string()))
            .or_insert(vec![l.to_string()]);
        connections
    })
}

fn pt1(connections: &HashMap<String, Vec<String>>) -> i32 {
    let mut interconnected: HashSet<Vec<String>> = HashSet::new();
    connections.iter().for_each(|(conn_1, conns_1)| {
        conns_1.iter().for_each(|conn_2| {
            if let Some(conns_2) = connections.get(conn_2) {
                conns_2.iter().for_each(|conn_3| {
                    if let Some(conns_3) = connections.get(conn_3) {
                        conns_3.iter().for_each(|conn_4| {
                            if conn_4 == conn_1 {
                                let mut network = vec![
                                    conn_1.to_string(),
                                    conn_2.to_string(),
                                    conn_3.to_string(),
                                ];
                                network.sort();
                                interconnected.insert(network);
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

fn pt2(connections: &HashMap<String, Vec<String>>) -> String {
    let mut queue: VecDeque<Vec<String>> =
        VecDeque::from_iter(connections.keys().map(|v| vec![v.to_string()]));
    let mut seen: HashSet<Vec<String>> = HashSet::new();

    while let Some(network) = queue.pop_front() {
        let mut computers = network.clone();
        network.iter().for_each(|conn_1| {
            if let Some(conns_1) = connections.get(conn_1) {
                conns_1.iter().for_each(|conn_2| {
                    if let Some(conns_2) = connections.get(conn_2) {
                        if computers.iter().all(|computer| conns_2.contains(computer)) {
                            computers.push(conn_2.to_string());
                            computers.sort();
                            if !seen.contains(&computers) {
                                seen.insert(computers.clone());
                                queue.push_back(computers.clone());
                            }
                        }
                    }
                });
            }
        });
    }

    let biggest_network = seen.iter().fold(vec![], |biggest, network| {
        if network.len() > biggest.len() {
            network.to_owned()
        } else {
            biggest
        }
    });

    biggest_network.join(",")
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let input = include_str!("./example.txt");
        let connections = parse(&input);
        let result = pt1(&connections);
        assert_eq!(result, 7);
    }

    #[test]
    fn pt2_test() {
        let input = include_str!("./example.txt");
        let connections = parse(&input);
        let result = pt2(&connections);
        assert_eq!(result, "co,de,ka,ta".to_string());
    }
}
