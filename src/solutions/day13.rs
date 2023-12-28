use itertools::Itertools;

type Pattern = Vec<Vec<char>>;
fn parse_input(input: &String) -> Vec<Pattern> {
    input
        .split("\n\n")
        .map(|pattern_str| {
            pattern_str
                .lines()
                .map(|v| v.chars().collect_vec())
                .collect_vec()
        })
        .collect_vec()
}

#[allow(unused)]
fn print_pattern(pattern: &Pattern) {
    let pattern_str = pattern
        .iter()
        .map(|v| v.iter().collect::<String>())
        .join("\n");
    println!("pattern =\n{}", pattern_str);
}

fn vecs_match<T>(v1: &Vec<T>, v2: &Vec<T>) -> bool
where
    T: PartialEq,
{
    v1.iter().zip(v2.iter()).all(|(v1, v2)| *v1 == *v2)
}

fn vecs_diff<T>(v1: &Vec<T>, v2: &Vec<T>) -> Vec<bool>
where
    T: PartialEq,
{
    v1.iter()
        .zip(v2.iter())
        .map(|(v1, v2)| *v1 != *v2)
        .collect_vec()
}

fn vecs_similar<T>(v1: &Vec<T>, v2: &Vec<T>) -> bool
where
    T: PartialEq,
{
    vecs_match(v1, v2) || vecs_diff(v1, v2).iter().filter(|v| **v == true).count() == 1
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

fn _find_exact_reflection_row_by(
    pattern: &Pattern,
    comp: fn(&Vec<char>, &Vec<char>) -> bool,
) -> Vec<usize> {
    let mut result = Vec::new();
    for i in 0..pattern.len() - 1 {
        if comp(&pattern[i], &pattern[i + 1]) {
            result.push(i);
        }
    }

    result
}

fn find_exact_reflections_row(pattern: &Pattern) -> Vec<usize> {
    _find_exact_reflection_row_by(&pattern, |v1, v2| vecs_match(v1, v2))
}

fn find_exact_reflections_col(pattern: &Pattern) -> Vec<usize> {
    let transposed_pattern = transpose(pattern);
    _find_exact_reflection_row_by(&transposed_pattern, |v1, v2| vecs_match(v1, v2))
}

fn _is_complete_row_reflection_from(pattern: &Pattern, i: usize) -> bool {
    let (mut i, mut j) = (i, i + 1);
    loop {
        if !vecs_match(&pattern[i], &pattern[j]) {
            break false;
        }

        if i == 0 || j == pattern.len() - 1 {
            break true;
        }

        i -= 1;
        j += 1;
    }
}

fn is_complete_row_reflection_from(pattern: &Pattern, i: usize) -> bool {
    _is_complete_row_reflection_from(pattern, i)
}

fn is_complete_col_reflection_from(pattern: &Pattern, i: usize) -> bool {
    let transposed_pattern = transpose(pattern);
    _is_complete_row_reflection_from(&transposed_pattern, i)
}

fn find_almost_exact_reflections_row(pattern: &Pattern) -> Vec<usize> {
    _find_exact_reflection_row_by(&pattern, |v1, v2| vecs_similar(v1, v2))
}

fn find_almost_exact_reflections_col(pattern: &Pattern) -> Vec<usize> {
    let transposed_pattern = transpose(pattern);
    _find_exact_reflection_row_by(&transposed_pattern, |v1, v2| vecs_similar(v1, v2))
}

fn _is_almost_complete_row_reflection_from(pattern: &Pattern, i: usize) -> bool {
    let (mut i, mut j) = (i, i + 1);
    let mut n_smudges = 0;

    loop {
        let diffs = vecs_diff(&pattern[i], &pattern[j])
            .iter()
            .filter(|v| **v == true)
            .count();
        n_smudges += if diffs == 1 { 1 } else { 0 };
        if diffs > 1 {
            break false;
        }

        if i == 0 || j == pattern.len() - 1 {
            break n_smudges == 1;
        }

        i -= 1;
        j += 1;
    }
}

fn is_almost_complete_row_reflection_from(pattern: &Pattern, i: usize) -> bool {
    _is_almost_complete_row_reflection_from(pattern, i)
}

fn is_almost_complete_col_reflection_from(pattern: &Pattern, i: usize) -> bool {
    let transposed_pattern = transpose(pattern);
    _is_almost_complete_row_reflection_from(&transposed_pattern, i)
}

pub fn p1(input: &String) -> String {
    let patterns = parse_input(input);
    let mut score = 0;
    for (_i, pattern) in patterns.iter().enumerate() {
        let row_reflections = find_exact_reflections_row(&pattern);
        let maybe_score = row_reflections
            .into_iter()
            .filter(|i| is_complete_row_reflection_from(pattern, *i))
            .map(|i| (i + 1) * 100)
            .last();

        score += match maybe_score {
            None => {
                let col_reflections = find_exact_reflections_col(&pattern);
                let maybe_score = col_reflections
                    .into_iter()
                    .filter(|i| is_complete_col_reflection_from(pattern, *i))
                    .last();
                maybe_score.unwrap() + 1
            }
            Some(score) => score,
        };
    }

    score.to_string()
}

pub fn p2(input: &String) -> String {
    let patterns = parse_input(input);
    let mut score = 0;
    for (_i, pattern) in patterns.iter().enumerate() {
        let row_reflections = find_almost_exact_reflections_row(&pattern);
        let maybe_score = row_reflections
            .into_iter()
            .filter(|i| is_almost_complete_row_reflection_from(pattern, *i))
            .map(|i| (i + 1) * 100)
            .find_or_first(|_| true);

        score += match maybe_score {
            None => {
                let col_reflections = find_almost_exact_reflections_col(&pattern);
                let maybe_score = col_reflections
                    .into_iter()
                    .filter(|i| is_almost_complete_col_reflection_from(pattern, *i))
                    .find_or_first(|_| true);
                let score = maybe_score.unwrap() + 1;
                score
            }
            Some(score) => score,
        };
    }

    score.to_string()
}
