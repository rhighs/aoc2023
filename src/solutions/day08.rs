use std::collections::HashMap;

use itertools::Itertools;

fn gcd(mut n: usize, mut m: usize) -> usize {
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

fn parse_input(input: &String) -> (String, Vec<(String, (String, String))>) {
    let input = input.replace("\r\n", "\n");
    let (directions, network) = input.split("\n\n").collect_tuple().unwrap();
    let network = network
        .lines()
        .map(|line| {
            let binding = line.replace(" ", "");
            let (source, ways) = binding
                .split("=")
                .map(|v| v.to_string())
                .collect_tuple()
                .unwrap();
            let (left, right) = ways
                .strip_prefix("(")
                .unwrap()
                .strip_suffix(")")
                .unwrap()
                .split(",")
                .map(|v| v.to_string())
                .collect_tuple()
                .unwrap();
            (source, (left, right))
        })
        .collect_vec();
    (directions.to_string(), network)
}

pub fn p1(input: &String) -> String {
    let (directions, network) = parse_input(input);
    let mut network_map = HashMap::<String, (String, String)>::new();
    for (name, (left, right)) in network {
        network_map.insert(name, (left, right));
    }

    let mut current = network_map[&String::from("AAA")].clone();
    let mut steps = 0usize;
    let mut cursor = 0usize;

    loop {
        let new_key = if directions.chars().nth(cursor).unwrap() == 'L' {
            current.0.clone()
        } else {
            current.1.clone()
        };
        cursor = (cursor + 1) % directions.len();
        steps += 1;
        if new_key == "ZZZ" {
            break;
        }
        current = network_map[&new_key].clone();
    }

    steps.to_string()
}

pub fn p2(input: &String) -> String {
    let (directions, network) = parse_input(input);
    let mut network_map = HashMap::<String, (String, String)>::new();
    for (name, (left, right)) in network {
        network_map.insert(name, (left, right));
    }

    let starting_nodes = network_map
        .keys()
        .filter(|k| k.ends_with("A"))
        .map(|s| s.to_string())
        .collect_vec();

    let mut cycles = Vec::<Vec<usize>>::new();

    for k in 0..starting_nodes.len() {
        let mut current = starting_nodes[k].clone();
        let mut cycle = Vec::new();
        let mut first_found: Option<String> = Option::None;

        let mut steps = 0;
        let mut cursor = 0;
        loop {
            while steps == 0 || !current.ends_with("Z") {
                steps += 1;
                current = if directions.chars().nth(cursor).unwrap() == 'L' {
                    network_map[&current].0.clone()
                } else {
                    network_map[&current].1.clone()
                };
                cursor = (cursor + 1) % directions.len();
            }

            cycle.push(steps);
            if first_found.is_some() && current == first_found.unwrap() {
                break;
            } else {
                first_found = Some(current.clone());
                steps = 0;
            }
        }

        cycles.push(cycle);
    }

    let lcm_between = cycles.iter().map(|v| *v.first().unwrap()).collect_vec();
    let mut lcm = (lcm_between[0] * lcm_between[1]) / gcd(lcm_between[0], lcm_between[1]);
    for k in lcm_between.iter().skip(2) {
        lcm = (lcm * *k) / gcd(lcm, *k);
    }
    lcm.to_string()
}
