use itertools::Itertools;
use std::collections::VecDeque;

type Map = Vec<Vec<Pipe>>;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Pipe {
    PipeStart,
    NorthToSouth,
    EastToWest,
    NorthToWest,
    NorthToEast,
    SouthToWest,
    SouthToEast,
    NoPipe,
    Empty,
}

#[derive(Clone, Copy)]
enum Dir {
    Top,
    Bottom,
    Left,
    Right,
}

fn create_2d_vector<T: Clone>(rows: usize, cols: usize, default_value: T) -> Vec<Vec<T>> {
    vec![vec![default_value; cols]; rows]
}

fn allow_from_direction(direction: Dir) -> [Pipe; 3] {
    use Pipe::*;
    match direction {
        Dir::Top => [NorthToSouth, NorthToEast, NorthToWest],
        Dir::Bottom => [NorthToSouth, SouthToEast, SouthToWest],
        Dir::Left => [EastToWest, NorthToWest, SouthToWest],
        Dir::Right => [EastToWest, NorthToEast, SouthToEast],
    }
}

fn parse_input(input: &String) -> Map {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '|' => Pipe::NorthToSouth,
                    '-' => Pipe::EastToWest,
                    'J' => Pipe::NorthToWest,
                    'L' => Pipe::NorthToEast,
                    '7' => Pipe::SouthToWest,
                    'F' => Pipe::SouthToEast,
                    'S' => Pipe::PipeStart,
                    '.' => Pipe::NoPipe,
                    _ => Pipe::Empty,
                })
                .collect_vec()
        })
        .collect_vec()
}

#[allow(unused)]
fn print_map(m: &Map) {
    for r in m {
        for c in r {
            print!(
                "{}",
                match c {
                    Pipe::NorthToSouth => '|',
                    Pipe::EastToWest => '-',
                    Pipe::NorthToWest => 'J',
                    Pipe::NorthToEast => 'L',
                    Pipe::SouthToWest => '7',
                    Pipe::SouthToEast => 'F',
                    Pipe::PipeStart => 'S',
                    Pipe::NoPipe => '.',
                    Pipe::Empty => ' ',
                }
            )
        }
        println!()
    }
}

fn traverse_map(pos: (usize, usize), map: &Map) -> Vec<Vec<usize>> {
    let mut q = VecDeque::<(usize, usize)>::new();
    let mut ds = create_2d_vector(map.len(), map[0].len(), 0usize);
    q.push_back(pos);

    while q.len() > 0 {
        let (ii, jj) = q.pop_front().unwrap();

        for ((i, j), direction) in vec![
            ((1, 0), Dir::Top),
            ((0, 1), Dir::Left),
            ((0, -1), Dir::Right),
            ((-1, 0), Dir::Bottom),
        ] {
            let (i, j) = ((ii as i64) + i, (jj as i64) + j);
            if i < 0 || j < 0 || i >= (map.len() as i64) || j >= (map[0].len() as i64) {
                continue;
            }
            let (i, j) = (i as usize, j as usize);
            if ds[i][j] > 0 {
                continue;
            }

            if allow_from_direction(direction).contains(&map[i][j]) {
                ds[i][j] = ds[ii][jj] + 1;
                q.push_back((i, j));
            }
        }
    }

    ds
}

fn find_start_position(map: &Map) -> (usize, usize) {
    map.iter()
        .enumerate()
        .find_map(|(i, row)| {
            if let Some(j) = row.iter().enumerate().find_map(|(i, tile)| match tile {
                Pipe::PipeStart => Some(i),
                _ => None,
            }) {
                Some((i, j))
            } else {
                None
            }
        })
        .unwrap()
}

pub fn p1(input: &String) -> String {
    let map = parse_input(input);
    let start_position = find_start_position(&map);
    let distances = traverse_map(start_position, &map);
    distances
        .into_iter()
        .flat_map(|row| row.into_iter())
        .max()
        .unwrap()
        .to_string()
}

fn fitting_pipe(map: &Map, (ii, jj): (usize, usize)) -> Pipe {
    let mut dirs = Vec::new();
    for ((i, j), from_direction, opposite) in vec![
        ((1, 0), Dir::Top, Dir::Bottom),
        ((0, 1), Dir::Left, Dir::Right),
        ((0, -1), Dir::Right, Dir::Left),
        ((-1, 0), Dir::Bottom, Dir::Top),
    ] {
        let (i, j) = ((ii as i64) + i, (jj as i64) + j);
        if i < 0 || j < 0 || i >= (map.len() as i64) || j >= (map[0].len() as i64) {
            continue;
        }
        let (i, j) = (i as usize, j as usize);
        if allow_from_direction(from_direction).contains(&map[i][j]) {
            dirs.push(opposite);
        }
    }
    if dirs.len() == 2 {
        if let Some(v) = allow_from_direction(dirs[0])
            .iter()
            .find(|v| allow_from_direction(dirs[1]).contains(v))
        {
            return *v;
        }
    }
    map[ii][jj]
}

fn upscale_map_2(m: &Map) -> Map {
    let mut new_map: Map = create_2d_vector(m.len() * 2 + 2, m[0].len() * 2 + 2, Pipe::NoPipe);
    for i in 0..m.len() {
        for j in 0..m[0].len() {
            new_map[1 + (i * 2)][1 + (j * 2)] = m[i][j];
        }
    }
    for i in 0..new_map.len() {
        for j in 0..new_map[0].len() {
            match new_map[i][j] {
                Pipe::NoPipe => new_map[i][j] = fitting_pipe(&new_map, (i, j)),
                _ => (),
            }
        }
    }
    new_map
}

fn downscale_map_2(m: &Map) -> Map {
    let mut new_map = create_2d_vector((m.len() - 2) / 2, (m[0].len() - 2) / 2, Pipe::NoPipe);
    for i in (1..m.len() - 1).step_by(2) {
        for j in (1..m[i].len() - 1).step_by(2) {
            let (ii, jj) = ((i - 1) / 2, (j - 1) / 2);
            new_map[ii][jj] = m[i][j];
        }
    }
    new_map
}

pub fn p2(input: &String) -> String {
    let mut map = parse_input(input);

    // Find and replace S with its fitting pipe and upscale the map by 2x.
    let start_position = find_start_position(&map);
    let start_pipe = fitting_pipe(&map, start_position);
    map[start_position.0][start_position.1] = start_pipe;
    let mut upscaled = upscale_map_2(&map);

    // Remove everything that is a Pipe::NoPipe via DFS.
    // No need to keep track of visits,
    // being != Pipe::NoPipe is considered a DFS visit
    {
        let mut stack = vec![(0, 0)];
        while stack.len() > 0 {
            let (ii, jj) = stack.pop().unwrap();
            match upscaled[ii][jj] {
                Pipe::NoPipe => (),
                _ => continue,
            }

            upscaled[ii][jj] = Pipe::Empty;

            for (i, j) in vec![(1, 0), (0, 1), (0, -1), (-1, 0)] {
                let (i, j) = ((ii as i64) + i, (jj as i64) + j);
                if i < 0 || j < 0 || i >= (upscaled.len() as i64) || j >= (upscaled[0].len() as i64)
                {
                    continue;
                }
                stack.push((i as usize, j as usize));
            }
        }
    }

    // Remove outside excess map by checking the sorroundings of a cell.
    {
        for i in 1..upscaled.len() - 1 {
            for j in 1..upscaled[i].len() - 1 {
                let dirs = vec![
                    (i, j - 1),
                    (i, j + 1),
                    (i + 1, j),
                    (i - 1, j),
                    (i - 1, j - 1),
                    (i - 1, j + 1),
                    (i + 1, j - 1),
                    (i + 1, j + 1),
                ];
                match upscaled[i][j] {
                    Pipe::NoPipe | Pipe::Empty => (),
                    _ => {
                        let mut has_any = false;
                        for (ii, jj) in dirs {
                            match upscaled[ii][jj] {
                                Pipe::NoPipe => {
                                    has_any = true;
                                    break;
                                }
                                _ => (),
                            }
                        }
                        if !has_any {
                            upscaled[i][j] = Pipe::Empty;
                        }
                    }
                }
            }
        }
    }

    // Fix up any artifact from previous step.
    {
        for i in 0..upscaled.len() {
            for j in 0..upscaled[i].len() {
                match upscaled[i][j] {
                    Pipe::Empty => upscaled[i][j] = fitting_pipe(&upscaled, (i, j)),
                    _ => (),
                }
            }
        }
    }

    // DFS visit empty cells and replace any first occurrence of a cell != Pipe::Empty
    // and Pipe::NoPipe as it is the border of our pipes main loop.
    // Removing the border will give us all that is contained inside of our main loop.
    {
        let mut stack = vec![(0, 0)];
        let mut vs = create_2d_vector(upscaled.len(), upscaled[0].len(), false);
        while stack.len() > 0 {
            let (ii, jj) = stack.pop().unwrap();
            if vs[ii][jj] {
                continue;
            }
            vs[ii][jj] = true;
            match upscaled[ii][jj] {
                Pipe::NoPipe => continue,
                _ => stack.push((ii, jj)),
            }

            upscaled[ii][jj] = Pipe::Empty;

            for (i, j) in vec![(1, 0), (0, 1), (0, -1), (-1, 0)] {
                let (i, j) = ((ii as i64) + i, (jj as i64) + j);
                if i >= 0
                    && j >= 0
                    && i < (upscaled.len() as i64)
                    && j < (upscaled[i as usize].len() as i64)
                {
                    stack.push((i as usize, j as usize));
                }
            }
        }
    }

    // Downscale the map back to its original size, any cell != Pipe::Empty will be part of the
    // area enclosed by the main loop.
    let downscaled = downscale_map_2(&upscaled);
    let count = downscaled
        .iter()
        .flat_map(|v| {
            v.iter().map(|v| match v {
                Pipe::Empty => 0,
                _ => 1,
            })
        })
        .sum::<usize>();

    count.to_string()
}
