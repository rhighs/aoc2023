use itertools::Itertools;

fn parse_input(input: &String) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|v| v.parse::<i32>().unwrap())
                .collect_vec()
        })
        .collect_vec()
}

fn find_gen_sum(histories: Vec<Vec<i32>>) -> i32 {
    let mut generated = Vec::new();
    for history in histories {
        let mut results: Vec<Vec<i32>> = vec![history.clone()];
        while !results.last().unwrap().iter().all(|&v| v == 0) {
            let last = results.last().unwrap();
            let result = (1..last.len()).map(|k| last[k] - last[k - 1]).collect_vec();
            results.push(result);
        }
        let mut history_gen: i32 = 0;
        for r in results.iter().rev() {
            history_gen = r.last().unwrap() + history_gen
        }
        generated.push(history_gen);
    }
    generated.iter().sum::<i32>()
}

pub fn p1(input: &String) -> String {
    let histories = parse_input(input);
    find_gen_sum(histories).to_string()
}

pub fn p2(input: &String) -> String {
    let histories = parse_input(input)
        .iter()
        .map(|v| v.iter().rev().cloned().collect_vec())
        .collect_vec();
    find_gen_sum(histories).to_string()
}
