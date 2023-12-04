use crate::aocdebug;

pub fn p1(input: &String) -> String {
    let mut result = 0u32;
    for line in input.trim().lines().into_iter() {
        let first = line.chars().find(|c| c.is_numeric())
            .unwrap_or('0')
            .to_digit(10)
            .unwrap();
        let second = line.chars().rev().find(|c| c.is_numeric())
            .unwrap_or('0')
            .to_digit(10)
            .unwrap();
        result += first * 10 + second;
    }
    result.to_string()
}

fn find_num_or_str_in(code: Vec<char>, search_in: &Vec<String>) -> u32 {
    let mut ns = String::from("");
    let mut k = 0;
    while k < code.len() {
        let c = code[k];
        if c.is_numeric() {
            return c.to_digit(10).unwrap()
        }

        ns.push(c);
        if !search_in.iter().any(|v| v.starts_with(ns.as_str())) {
            k -= ns.len() - 1;
            ns.clear();
        } else if let Some((i, _)) = search_in.iter().enumerate().find(
            |(_, v)| ns == **v
        ) {
            return i as u32
        }
        k +=1
    }

    unreachable!()
}

pub fn p2(input: &String) -> String {
    let mut result = 0u32;
    let numbers_str = vec![
        "_", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"
    ].into_iter().map(|v| String::from(v)).collect();

    for line in input.trim().lines().into_iter() {
        let first = find_num_or_str_in(line.chars().collect(), &numbers_str);
        let second = find_num_or_str_in(line.chars().rev().collect(), 
            &numbers_str.iter().map(
                |each| each.chars().rev().collect()
            ).collect()
        );
        result += first * 10 + second;

        if aocdebug!() {
            println!("{} {}", first * 10 + second, line);
        }
    }

    result.to_string()
}