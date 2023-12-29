use itertools::Itertools;
use std::collections::HashMap;

type Base = Vec<Vec<char>>;
fn parse_input(input: &String) -> Base {
    input.lines().map(|line| line.chars().collect_vec()).collect_vec()
}

#[allow(unused)]
fn print_base(base: &Base) {
    let base_str = base
        .iter()
        .map(|v| v.iter().collect::<String>())
        .join("\n");
    println!("base =\n{}", base_str);
}

fn transpose<T>(v: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

fn slide_row_left(row: &mut Vec<char>) {
    let mut i = 0;
    let mut new_row = vec!['#'; row.len()];

    while i < row.len() {
        let mut j = i;
        let mut n = 0;
        let start_at = i;
        while j < row.len() && row[j] != '#' {
            if row[j] == 'O' {
                n += 1;
            }
            j += 1;
        }

        for k in start_at..j {
            if n > 0 {
                new_row[k] = 'O';
                n -= 1;
            } else {
                new_row[k] = '.';
            }
        }
        if j+1 < row.len() {
            new_row[j+1] = '#';
        }

        i = j+1;
    }

    for i in 0..row.len() {
        row[i] = new_row[i];
    }
}

fn slide_row_right(row: &mut Vec<char>) {
    let mut i = row.len();
    let mut new_row = vec!['#'; row.len()];

    while i > 0 {
        let mut j = i;
        let mut n = 0;
        let end_at = i;
        while j > 0 && row[j - 1] != '#' {
            if row[j - 1] == 'O' {
                n += 1;
            }
            j -= 1;
        }

        for k in (j..end_at).rev() {
            if n > 0 {
                new_row[k] = 'O';
                n -= 1;
            } else {
                new_row[k] = '.';
            }
        }
        if j > 0 {
            new_row[j - 1] = '#';
        }

        if j == 0 {
            break;
        }
        i = j - 1;
    }

    for i in 0..row.len() {
        row[i] = new_row[i];
    }
}

fn row_weight(row: &Vec<char>) -> usize {
    row.iter().enumerate().map(|(i, &v)| if v == 'O' { row.len() - i} else { 0 }).sum()
}

pub fn p1(input: &String) -> String {
    let rocks = parse_input(input);
    let mut rocks = transpose(&rocks);
    for i in 0..rocks.len() {
        slide_row_left(&mut rocks[i]);
    }
    let result = rocks.iter().map(|row| row_weight(row)).sum::<usize>();

    result.to_string()
}

pub fn p2(input: &String) -> String {
    let mut rocks = parse_input(input);
    let mut memo: HashMap<Base, usize> = HashMap::new();

    let mut c = 0;
    let limit = 1000000000;
    while c < limit {
        if let Some(l) = memo.get(&rocks) {
            let loop_length = c - l;
            c += ((limit-c)/loop_length) * loop_length;
            memo.clear();
            continue;
        }

        let orig = rocks.clone();
        rocks = transpose(&rocks);
        for i in 0..rocks.len() {
            slide_row_left(&mut rocks[i]);
        }
        rocks = transpose(&rocks);
        for i in 0..rocks.len() {
            slide_row_left(&mut rocks[i]);
        }
        rocks = transpose(&rocks);
        for i in 0..rocks.len() {
            slide_row_right(&mut rocks[i]);
        }
        rocks = transpose(&rocks);
        for i in 0..rocks.len() {
            slide_row_right(&mut rocks[i]);
        }
        memo.insert(orig, c);
        c += 1;
    }

    let rocks = transpose(&rocks);
    let result = rocks.iter().map(|row| row_weight(row)).sum::<usize>();
    result.to_string()
}
