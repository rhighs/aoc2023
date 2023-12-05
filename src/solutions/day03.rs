use std::collections::HashSet;

fn parse_number(grid: &Vec<Vec<char>>, i: usize, j: usize) -> (u32, usize) {
    let result: String = grid[i]
        .iter()
        .skip(j)
        .take_while(|c| c.is_numeric())
        .collect();
    (result.parse::<u32>().unwrap(), result.len())
}

fn find_adjs_by<F>(grid: &Vec<Vec<char>>, i: usize, j: usize, adj_rule: F) -> Vec<(usize, usize)>
where
    F: Fn(char) -> bool,
{
    let mut result = Vec::new();
    let (ii, jj) = (i, j);
    for i in (if i > 0 { i - 1 } else { i })..=(if i < grid.len() - 1 { i + 1 } else { i }) {
        for j in (if j > 0 { j - 1 } else { j })..=(if j < grid[i].len() - 1 { j + 1 } else { j }) {
            if !(ii == i && jj == j) && adj_rule(grid[i][j]) {
                result.push((i, j));
            }
        }
    }
    result
}

fn part_has_adj_special(grid: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    if j >= grid[i].len() || !grid[i][j].is_numeric() {
        return false;
    }

    if find_adjs_by(grid, i, j, |c| c != '.' && !c.is_numeric()).len() > 0 {
        return true;
    }

    part_has_adj_special(grid, i, j + 1)
}

fn find_gear_adj_parts(grid: &Vec<Vec<char>>, i: usize, j: usize) -> Option<Vec<u32>> {
    if grid[i][j] != '*' {
        return None;
    }
    let part_positions = find_adjs_by(grid, i, j, |c| c.is_numeric());
    Some(
        HashSet::<u32>::from_iter(
            part_positions
                .into_iter()
                .map(|(i, j)| find_whole_number(&grid, i, j))
                .into_iter(),
        )
        .into_iter()
        .collect(),
    )
}

pub fn p1(input: &String) -> String {
    let grid: Vec<Vec<char>> = input.lines().map(|v| v.chars().collect()).collect();
    let mut numbers = Vec::new();

    for i in 0..grid.len() {
        let mut skipj = 0;
        for j in 0..grid[i].len() {
            if skipj > 0 {
                skipj -= 1;
                continue;
            }

            if grid[i][j].is_numeric() && part_has_adj_special(&grid, i, j) {
                let (n, skip) = parse_number(&grid, i, j);
                numbers.push(n);
                skipj = skip;
            }
        }
    }

    numbers.iter().sum::<u32>().to_string()
}

fn find_whole_number(grid: &Vec<Vec<char>>, i: usize, j: usize) -> u32 {
    let digits_back: String = grid[i]
        .iter()
        .skip(j + 1)
        .take_while(|c| c.is_numeric())
        .cloned()
        .collect::<String>()
        .chars()
        .collect();
    let digits_front: String = grid[i]
        .iter()
        .rev()
        .skip(grid[i].len() - j - 1)
        .take_while(|c| c.is_numeric())
        .cloned()
        .collect::<String>()
        .chars()
        .rev()
        .collect();
    (digits_front + digits_back.as_str()).parse().unwrap()
}

pub fn p2(input: &String) -> String {
    let grid: Vec<Vec<char>> = input.lines().map(|v| v.chars().collect()).collect();
    let mut result = 0;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if let Some(parts) = find_gear_adj_parts(&grid, i, j) {
                if parts.len() == 2 {
                    result += parts.iter().fold(1, |acc, v| acc * v);
                }
            }
        }
    }

    result.to_string()
}
