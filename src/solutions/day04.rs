use itertools::Itertools;

type Card = (Vec<u32>, Vec<u32>);

fn parse_input(input: &String) -> Vec<Card> {
    input
        .lines()
        .into_iter()
        .map(|line|
            line
                .chars()
                .skip_while(|c| *c != ':')
                .skip(1)
                .collect::<String>()
        )
        .map(|v|
            v
                .split("|")
                .map(|v|
                    v
                        .split_whitespace()
                        .map(|v|
                            v
                                .trim()
                                .parse::<u32>()
                                .expect(format!("couldn't parse u32 value from = ({})", v).as_str())
                        )
                        .collect::<Vec<u32>>()
                )
                .collect_tuple()
                .unwrap()
        )
        .collect::<Vec<Card>>()
}

pub fn p1(input: &String) -> String {
    let cards = parse_input(input);
    let count: u32 = cards
        .iter()
        .map(|(winning, mine)|
            mine
                .iter()
                .filter(|m| winning.contains(m))
                .count()
        )
        .filter(|&c| c > 0)
        .map(|c| (2u32).pow((c as u32) - 1))
        .sum();
    count.to_string()
}

type CardInstance = (Vec<u32>, Vec<u32>, u32);

pub fn p2(input: &String) -> String {
    let cards: Vec<CardInstance> = parse_input(input)
        .into_iter()
        .map(|(w, m)| (w, m, 0))
        .collect();
    String::new()
}
