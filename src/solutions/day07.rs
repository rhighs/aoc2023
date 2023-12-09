use std::{cmp::Ord, collections::HashMap};

use itertools::Itertools;

fn parse_input(input: &String) -> Vec<(String, usize)> {
    input
        .lines()
        .map(|v| v.split(" ").collect_tuple().unwrap())
        .map(|(v1, v2)| (v1.to_string(), v2.parse::<usize>().unwrap()))
        .collect_vec()
}

fn card_value(c: char, p2: bool) -> usize {
    match c {
        '1'..='9' => c.to_digit(10).unwrap() as usize,
        'T' => 10,
        'J' => {
            if p2 {
                0
            } else {
                11
            }
        }
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => unreachable!(),
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd)]
enum HandKind {
    Five(Vec<usize>),
    Four(Vec<usize>),
    FullHouse(Vec<usize>),
    Three(Vec<usize>),
    TwoPair(Vec<usize>),
    OnePair(Vec<usize>),
    HighCard(Vec<usize>),
}

impl Ord for HandKind {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let cmp_vec = |v1: &Vec<usize>, v2: &Vec<usize>| {
            v1.iter()
                .zip(v2.iter())
                .skip_while(|(v1, v2)| v1.cmp(v2).is_eq())
                .map(|(v1, v2)| v1.cmp(v2))
                .find_or_first(|_| true)
                .unwrap()
        };
        match self {
            HandKind::Five(otherv) => match other {
                HandKind::Five(otherv1) => cmp_vec(otherv, &otherv1),
                _ => std::cmp::Ordering::Greater,
            },
            HandKind::Four(otherv) => match other {
                HandKind::Five(_) => std::cmp::Ordering::Less,
                HandKind::Four(otherv1) => cmp_vec(otherv, otherv1),
                _ => std::cmp::Ordering::Greater,
            },
            HandKind::FullHouse(otherv) => match other {
                HandKind::Five(..) | HandKind::Four(..) => std::cmp::Ordering::Less,
                HandKind::FullHouse(otherv1) => cmp_vec(otherv, otherv1),
                _ => std::cmp::Ordering::Greater,
            },
            HandKind::Three(otherv) => match other {
                HandKind::Five(..) | HandKind::FullHouse(..) | HandKind::Four(..) => {
                    std::cmp::Ordering::Less
                }
                HandKind::Three(otherv1) => cmp_vec(otherv, otherv1),
                _ => std::cmp::Ordering::Greater,
            },
            HandKind::TwoPair(otherv) => match other {
                HandKind::Five(..)
                | HandKind::FullHouse(..)
                | HandKind::Four(..)
                | HandKind::Three(..) => std::cmp::Ordering::Less,
                HandKind::TwoPair(otherv1) => cmp_vec(otherv, otherv1),
                _ => std::cmp::Ordering::Greater,
            },
            HandKind::OnePair(otherv) => match other {
                HandKind::Five(..)
                | HandKind::Four(..)
                | HandKind::Three(..)
                | HandKind::FullHouse(..)
                | HandKind::TwoPair(..) => std::cmp::Ordering::Less,
                HandKind::OnePair(otherv1) => cmp_vec(otherv, otherv1),
                _ => std::cmp::Ordering::Greater,
            },
            HandKind::HighCard(otherv) => match other {
                HandKind::HighCard(otherv1) => cmp_vec(otherv, otherv1),
                _ => std::cmp::Ordering::Less,
            },
        }
    }

    fn max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        if self.cmp(&other).is_ge() {
            self
        } else {
            other
        }
    }

    fn min(self, other: Self) -> Self
    where
        Self: Sized,
    {
        if self.cmp(&other).is_le() {
            self
        } else {
            other
        }
    }
}

impl HandKind {
    fn upgrade(self) -> HandKind {
        let j: usize = 0;
        match self {
            HandKind::Five(values) => HandKind::Five(values),
            HandKind::Four(values) => {
                let nj = values.iter().filter(|&&c| c == j).count();
                match nj {
                    1 | 4 => HandKind::Five(values),
                    _ => HandKind::Four(values),
                }
            }
            HandKind::FullHouse(values) => {
                let nj = values.iter().filter(|&&c| c == j).count();
                match nj {
                    2 | 3 => HandKind::Five(values),
                    _ => HandKind::FullHouse(values),
                }
            }
            HandKind::Three(values) => {
                let nj = values.iter().filter(|&&c| c == j).count();
                match nj {
                    1 | 3 => HandKind::Four(values),
                    _ => HandKind::Three(values),
                }
            }
            HandKind::TwoPair(values) => {
                let nj = values.iter().filter(|&&c| c == j).count();
                match nj {
                    1 => HandKind::FullHouse(values),
                    2 => HandKind::Four(values),
                    _ => HandKind::TwoPair(values),
                }
            }
            HandKind::OnePair(values) => {
                let nj = values.iter().filter(|&&c| c == j).count();
                match nj {
                    1 | 2 => HandKind::Three(values),
                    _ => HandKind::OnePair(values),
                }
            }
            HandKind::HighCard(values) => {
                let nj = values.iter().filter(|&&c| c == j).count();
                match nj {
                    1 => HandKind::OnePair(values),
                    _ => HandKind::HighCard(values),
                }
            }
        }
    }
}

fn hand_kind(card_values: Vec<usize>) -> HandKind {
    let grouped = card_values
        .iter()
        .cloned()
        .fold(HashMap::<usize, usize>::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        });
    let values = grouped
        .iter()
        .sorted_by(|(_, v1), (_, v2)| v2.cmp(v1))
        .map(|(_, &v)| v)
        .collect_vec();

    match values.as_slice() {
        [5, ..] => HandKind::Five(card_values),
        [4, ..] => HandKind::Four(card_values),
        [3, 2, ..] => HandKind::FullHouse(card_values),
        [3, ..] => HandKind::Three(card_values),
        [2, 2, _v1] => HandKind::TwoPair(card_values),
        [2, _v1, _v2, _v3] => HandKind::OnePair(card_values),
        _ => HandKind::HighCard(card_values),
    }
}

pub fn p1(input: &String) -> String {
    let hands = parse_input(input);
    hands
        .iter()
        .map(|(v, b)| {
            (
                hand_kind(v.chars().map(|v| card_value(v, false)).collect_vec()),
                b,
            )
        })
        .sorted_by(|(h1, _), (h2, _)| h1.cmp(h2))
        .enumerate()
        .map(|(i, (_, b))| (i + 1) * b)
        .reduce(|a, b| a + b)
        .unwrap()
        .to_string()
}

pub fn p2(input: &String) -> String {
    let hands = parse_input(input);
    hands
        .iter()
        .map(|(v, b)| {
            (
                hand_kind(v.chars().map(|v| card_value(v, true)).collect_vec()),
                b,
            )
        })
        .map(|(h, b)| (h.upgrade(), b))
        .sorted_by(|(h1, _), (h2, _)| h1.cmp(h2))
        .enumerate()
        .map(|(i, (_, b))| (i + 1) * b)
        .reduce(|a, b| a + b)
        .unwrap()
        .to_string()
}
