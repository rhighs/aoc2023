mod solutions;
use solutions::*;

type AOCSolutions = (fn(&String) -> String, fn(&String) -> String);

static SOLUTION_FUNCS: [AOCSolutions; 25] = [
    (day01::p1, day01::p2),
    (day02::p1, day02::p2),
    (day03::p1, day03::p2),
    (day04::p1, day04::p2),
    (day05::p1, day05::p2),
    (day06::p1, day06::p2),
    (day07::p1, day07::p2),
    (day08::p1, day08::p2),
    (day09::p1, day09::p2),
    (day10::p1, day10::p2),
    (day11::p1, day11::p2),
    (day12::p1, day12::p2),
    (day13::p1, day13::p2),
    (day14::p1, day14::p2),
    (day15::p1, day15::p2),
    (day16::p1, day16::p2),
    (day17::p1, day17::p2),
    (day18::p1, day18::p2),
    (day19::p1, day19::p2),
    (day20::p1, day20::p2),
    (day21::p1, day21::p2),
    (day22::p1, day22::p2),
    (day23::p1, day23::p2),
    (day24::p1, day24::p2),
    (day25::p1, day25::p2),
];

fn read_file(input_filepath: &String) -> String {
    std::fs::read_to_string(input_filepath).expect(
        format!("{} no such file was found", input_filepath).as_str()
    )
}

fn main() {
    let problem_number = std::env::args().nth(1).expect("no problem number was passed");
    let problem_number = problem_number.parse::<usize>().expect(
        format!("problem_number = {} is not a valid value", problem_number).as_str()
    );
    assert!(problem_number < 25, "problem number must be within 1..=25 got = {}", problem_number);
    let input_filepath = std::env::args().nth(2).expect("no problem input file was given");

    let only_part = std::env::args().nth(3).unwrap_or(
        String::from("0")
    ).parse::<usize>().unwrap_or(0);
    assert!(only_part < 3, "part number cannot be > 2");

    let input = read_file(&input_filepath);

    match only_part {
        0 => {
            let s1 = SOLUTION_FUNCS[problem_number-1].0(&input);
            let s2 = SOLUTION_FUNCS[problem_number-1].1(&input);
            println!("p1 = {}, p2 = {}", s1, s2);
        },
        1 => {
            let s1 = SOLUTION_FUNCS[problem_number-1].0(&input);
            println!("p1 = {}", s1);
        },
        2 => {
            let s2 = SOLUTION_FUNCS[problem_number-1].1(&input);
            println!("p2 = {}", s2);
        }
        _ => unreachable!()
    }
}
