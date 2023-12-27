use itertools::Itertools;
use std::collections::HashMap;

fn parse_input(input: &String) -> Vec<(Vec<char>, Vec<usize>)> {
    input
        .lines()
        .map(|line| line.split_whitespace().collect_tuple().unwrap())
        .map(|(spring, arr)| {
            (
                spring.chars().collect_vec(),
                arr.split(",")
                    .map(|v| v.parse::<usize>().unwrap())
                    .collect_vec(),
            )
        })
        .collect_vec()
}

#[allow(unused)]
fn do_vecs_match<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
    let matching = a.iter().zip(b.iter()).filter(|&(a, b)| a == b).count();
    matching == a.len() && matching == b.len()
}

#[allow(unused)]
fn validate_against(springs: &Vec<char>, placement: &Vec<usize>) -> bool {
    let result_placement = springs
        .split(|c| *c != '#')
        .filter(|s| !s.is_empty())
        .map(|s| s.len())
        .collect_vec();
    do_vecs_match(&result_placement, placement)
}

#[allow(unused)]
pub fn _brute_force_p1(input: &String) -> String {
    let input_data = parse_input(input);
    input_data
        .iter()
        .map(|(spring_template, placement)| {
            let mut result = Vec::new();
            let pos = spring_template
                .iter()
                .enumerate()
                .filter(|(_, c)| **c == '?')
                .map(|(i, _)| i)
                .collect_vec();
            let nunknown = pos.len();
            for i in 0..2usize.pow(nunknown as u32) {
                let mut item: Vec<char> = spring_template.clone();
                let mut v = i;
                for p in &pos {
                    if item[*p] == '?' {
                        item[*p] = if v & 0x1 == 1 { '#' } else { '.' };
                        v >>= 1;
                    } else {
                        continue;
                    }
                }
                result.push(item);
            }
            result
                .iter()
                .filter(|value| validate_against(value, &placement))
                .count()
        })
        .sum::<usize>()
        .to_string()
}

fn recurse_pattern(
    mut pattern: Vec<char>,
    mut springs: Vec<usize>,
    pos: usize,
    running: bool,
    memo: &mut HashMap<(Vec<char>, Vec<usize>, usize, bool), usize>,
) -> usize {
    let key = (
        pattern.iter().skip(pos).cloned().collect(),
        springs.clone(),
        pos,
        running,
    );
    if let Some(&result) = memo.get(&key) {
        return result;
    }

    let result =
        if springs.is_empty() && pattern.iter().skip(pos).filter(|c| **c == '#').count() == 0 {
            1
        } else if pos >= pattern.len() {
            0
        } else {
            match pattern[pos] {
                '?' => {
                    let mut r1 = 0;
                    if !running {
                        pattern[pos] = '.';
                        r1 = recurse_pattern(pattern.clone(), springs.clone(), pos, running, memo);
                    }

                    pattern[pos] = '#';
                    let r2 = recurse_pattern(pattern.clone(), springs.clone(), pos, running, memo);
                    r1 + r2
                }
                '#' => {
                    if springs.is_empty() || springs[0] == 0 {
                        0
                    } else {
                        springs[0] -= 1;
                        let running = springs[0] > 0;
                        recurse_pattern(pattern.clone(), springs.clone(), pos + 1, running, memo)
                    }
                }
                '.' => {
                    if !springs.is_empty() && springs[0] == 0 {
                        springs.remove(0);
                        recurse_pattern(pattern.clone(), springs.clone(), pos + 1, false, memo)
                    } else if !springs.is_empty() && springs[0] > 0 && running {
                        0
                    } else {
                        recurse_pattern(pattern.clone(), springs.clone(), pos + 1, false, memo)
                    }
                }
                _ => unreachable!(),
            }
        };

    memo.insert(key, result);
    result
}

fn replace_contiguous_duplicates<T: PartialEq + Clone>(vec: Vec<T>, value: T) -> Vec<T> {
    let mut result = Vec::new();
    for item in vec {
        if result.last() != Some(&value) || item != value {
            result.push(item);
        }
    }
    result
}

fn repeat_with_sep<T: PartialEq + Clone>(vec: Vec<T>, repeat: usize, sep: T) -> Vec<T> {
    (0..repeat)
        .map(|v| {
            if v != repeat - 1 {
                let mut v = vec.clone();
                v.push(sep.clone());
                v
            } else {
                vec.clone()
            }
        })
        .flat_map(|v| v.into_iter())
        .collect_vec()
}

fn solve(input: &String, repeat_times: usize) -> usize {
    let input_data = parse_input(input)
        .into_iter()
        .map(|(p, s)| {
            let mut repeated_pattern = repeat_with_sep(p, repeat_times, '?');
            let repeated_springs = s.repeat(repeat_times);
            repeated_pattern.push('.');
            let result = (
                replace_contiguous_duplicates(repeated_pattern, '.'),
                repeated_springs,
            );
            result
        })
        .collect_vec();
    let mut memo = HashMap::new();
    input_data
        .into_iter()
        .map(|(p, s)| recurse_pattern(p, s, 0, false, &mut memo))
        .sum::<usize>()
}

pub fn p1(input: &String) -> String {
    solve(input, 1).to_string()
}

pub fn p2(input: &String) -> String {
    solve(input, 5).to_string()
}
