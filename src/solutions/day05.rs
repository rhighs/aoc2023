use itertools::Itertools;

fn parse_input(input: &String) -> (Vec<usize>, Mappings) {
    (
        input
            .lines()
            .find_or_first(|_| true)
            .unwrap()
            .chars()
            .skip_while(|&c| c != ':')
            .skip(1)
            .collect::<String>()
            .split_whitespace()
            .map(|c| c.parse::<usize>().unwrap())
            .collect::<Vec<usize>>(),
        input
            .split("\n\n")
            .skip(1)
            .map(
                |data| match data.lines().collect::<Vec<&str>>().as_slice() {
                    [_heading, tail @ ..] => tail
                        .iter()
                        .map(|line| {
                            let (destination, source, range) = line
                                .split(" ")
                                .map(|v| v.parse::<usize>().unwrap())
                                .collect_tuple()
                                .unwrap();
                            Mapping::new_with(source, destination, range)
                        })
                        .collect::<Vec<Mapping>>(),
                    _ => unreachable!(),
                },
            )
            .collect::<Vec<Vec<Mapping>>>(),
    )
}

#[derive(Debug)]
struct Mapping {
    from: std::ops::Range<usize>,
    to: std::ops::Range<usize>,
}

impl Mapping {
    fn new_with(fstart: usize, dstart: usize, len: usize) -> Self {
        Mapping {
            from: fstart..fstart + len,
            to: dstart..dstart + len,
        }
    }

    fn map(&self, n: usize) -> Option<usize> {
        if self.from.contains(&n) {
            Some(
                self.to
                    .clone()
                    .skip(n - self.from.start)
                    .find_or_first(|_| true)
                    .unwrap(),
            )
        } else {
            None
        }
    }
}

type TypeConversions = Vec<Mapping>;
type Mappings = Vec<TypeConversions>;
fn apply_maps(ms: &Mappings, n: usize) -> usize {
    let mut result = n;
    for tc in ms {
        for t in tc {
            if let Some(r) = t.map(result) {
                result = r;
                break;
            }
        }
    }
    result
}

pub fn p1(input: &String) -> String {
    let (seeds, mappings) = parse_input(input);
    seeds
        .iter()
        .map(|&s| apply_maps(&mappings, s))
        .min()
        .unwrap()
        .to_string()
}

pub fn p2(input: &String) -> String {
    let (seeds, mappings) = parse_input(input);
    seeds
        .chunks(2)
        .map(|chunk| (chunk[0]..chunk[1] + chunk[0]))
        .map(|s| s.map(|s| apply_maps(&mappings, s)).min().unwrap())
        .min()
        .unwrap()
        .to_string()
}
