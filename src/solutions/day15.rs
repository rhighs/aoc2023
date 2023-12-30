use std::collections::VecDeque;

use itertools::Itertools;

fn hash(current_value: usize, salt: char) -> usize {
    ((current_value + salt as usize) * 17) % 256
}

pub fn p1(input: &String) -> String {
    input
        .split(",")
        .map(|line| line.chars().filter(|v| *v != '\n').collect_vec())
        .map(|chars| chars.iter().fold(0, |acc, v| hash(acc, *v)))
        .sum::<usize>()
        .to_string()
}

pub fn p2(input: &String) -> String {
    let mut boxes = vec![VecDeque::<(String, usize)>::new(); 256];
    input
        .split(",")
        .map(|line| {
            let line = line.chars().filter(|c| *c != '\n').collect::<String>();
            if line.contains('=') {
                let (left, right) = line
                    .split('=')
                    .map(|s| s.to_owned())
                    .collect_tuple()
                    .unwrap();
                (left, right.parse::<usize>().unwrap(), '=')
            } else {
                let (left, _) = line
                    .split('-')
                    .map(|s| s.to_owned())
                    .collect_tuple()
                    .unwrap();
                (left, 0usize, '-')
            }
        })
        .for_each(|(label, focal, op)| {
            let box_number = label.chars().fold(0, |acc, v| hash(acc, v));
            match op {
                '=' => {
                    let any = boxes[box_number].iter().find_position(|(s, _)| *s == label);
                    if let Some((index, (_, _))) = any {
                        boxes[box_number][index] = (label, focal);
                    } else {
                        boxes[box_number].push_back((label, focal));
                    }
                }
                '-' => {
                    if let Some((index, _)) =
                        boxes[box_number].iter().find_position(|(s, _)| *s == label)
                    {
                        boxes[box_number].remove(index);
                    }
                }
                _ => unreachable!(),
            }
        });

    boxes
        .iter()
        .enumerate()
        .map(|(box_number, boxx)| (box_number + 1, boxx))
        .flat_map(|(box_number, boxx)| {
            boxx.iter()
                .enumerate()
                .map(|(slot_number, (label, focal))| (slot_number + 1, (label, focal)))
                .map(move |(slot_number, (_, focal))| box_number * slot_number * focal)
        })
        .sum::<usize>()
        .to_string()
}
