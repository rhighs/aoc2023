use itertools::Itertools;

fn parse_input_1(input: &String) -> Vec<(usize, usize)> {
    let (t, d) = input
        .lines()
        .map(|v| {
            v.chars()
                .skip_while(|&c| c != ':')
                .skip(1)
                .collect::<String>()
        })
        .map(|s| {
            s.split_whitespace()
                .map(|v| v.parse::<usize>().unwrap())
                .collect_vec()
        })
        .collect_tuple()
        .unwrap();
    t.into_iter().zip(d).collect_vec()
}

fn parse_input_2(input: &String) -> (usize, usize) {
    let (t, d) = input
        .lines()
        .map(|v| {
            v.chars()
                .skip_while(|&c| c != ':')
                .skip(1)
                .collect::<String>()
        })
        .map(|s| {
            s.chars()
                .filter(|&c| c != ' ')
                .collect::<String>()
                .parse::<usize>()
        })
        .collect_tuple()
        .unwrap();
    t.into_iter().zip(d).collect_vec().pop().unwrap()
}

fn ways((t, d): (usize, usize)) -> usize {
    (1..=(t / 2)).fold(0, |acc, v| {
        if v * (t - v) > d {
            acc + if v == (t - v) { 1 } else { 2 }
        } else {
            0
        }
    })
}

pub fn p1(input: &String) -> String {
    parse_input_1(input)
        .iter()
        .fold(1, |acc, &v| acc * ways(v))
        .to_string()
}

pub fn p2(input: &String) -> String {
    ways(parse_input_2(input)).to_string()
}
