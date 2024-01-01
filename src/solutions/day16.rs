use itertools::Itertools;

type Field = Vec<Vec<char>>;
type FieldEnergy = Vec<Vec<u8>>;

fn parse_input(input: &String) -> Field {
    input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec()
}

#[allow(unused)]
fn print_field(f: &Field) {
    let f_str = f.iter().map(|v| v.iter().collect::<String>()).join("\n");
    println!("{}", f_str);
}

#[derive(Debug)]
enum Dir {
    Up,
    Down,
    Right,
    Left,
}

impl Dir {
    fn offset(&self) -> (i32, i32) {
        use Dir::*;
        match self {
            Up => (0, -1),
            Down => (0, 1),
            Right => (1, 0),
            Left => (-1, 0),
        }
    }

    fn rotate_90_ccw(&self) -> Dir {
        use Dir::*;
        match self {
            Up => Left,
            Down => Right,
            Right => Down,
            Left => Up,
        }
    }

    fn rotate_90_cw(&self) -> Dir {
        use Dir::*;
        match self {
            Up => Right,
            Down => Left,
            Right => Up,
            Left => Down,
        }
    }

    fn vertical(&self) -> bool {
        use Dir::*;
        match self {
            Up | Down => true,
            _ => false,
        }
    }

    fn horizontal(&self) -> bool {
        !self.vertical()
    }

    fn flag(&self) -> u8 {
        use Dir::*;
        match self {
            Up => 0b0001,
            Down => 0b0010,
            Right => 0b0100,
            Left => 0b1000,
        }
    }
}

fn shoot_beam((x, y): (i32, i32), dir: Dir, field: &Field, field_energy: &mut FieldEnergy) {
    let (ox, oy) = dir.offset();
    let (x, y) = (ox + x, oy + y);
    if x < 0 || x as usize >= field[0].len() || y < 0 || y as usize >= field.len() {
        return;
    }
    let (xu, yu) = (x as usize, y as usize);
    let dir_flag = dir.flag();
    if field_energy[yu][xu] & dir_flag > 0 {
        return;
    }
    field_energy[yu][xu] |= dir_flag;

    let pos = (x, y);
    match field[yu][xu] {
        '.' => shoot_beam(pos, dir, field, field_energy),
        '|' => {
            if dir.vertical() {
                shoot_beam(pos, dir, field, field_energy)
            } else {
                shoot_beam(pos, Dir::Down, field, field_energy);
                shoot_beam(pos, Dir::Up, field, field_energy)
            }
        }
        '-' => {
            if dir.horizontal() {
                shoot_beam(pos, dir, field, field_energy)
            } else {
                shoot_beam(pos, Dir::Left, field, field_energy);
                shoot_beam(pos, Dir::Right, field, field_energy)
            }
        }
        '\\' => shoot_beam(pos, dir.rotate_90_ccw(), field, field_energy),
        '/' => shoot_beam(pos, dir.rotate_90_cw(), field, field_energy),
        _ => unreachable!(),
    }
}

fn solve_from(field: &Field, start: (i32, i32), dir: Dir) -> usize {
    let mut fe = vec![vec![0u8; field[0].len()]; field.len()];
    shoot_beam(start, dir, &field, &mut fe);
    fe.iter()
        .flat_map(|row| row.iter())
        .filter(|v| **v > 0)
        .count()
}

pub fn p1(input: &String) -> String {
    let field = parse_input(input);
    solve_from(&field, (-1, 0), Dir::Right).to_string()
}

pub fn p2(input: &String) -> String {
    let field = parse_input(input);
    (0..field[0].len())
        .map(|v| ((v as i32, -1i32), Dir::Down))
        .chain((0..field[0].len()).map(|v| ((v as i32, field.len() as i32), Dir::Up)))
        .chain((0..field.len()).map(|v| ((-1, v as i32), Dir::Right)))
        .chain((0..field.len()).map(|v| ((field[0].len() as i32, v as i32), Dir::Left)))
        .map(|(start, dir)| solve_from(&field, start, dir))
        .max()
        .unwrap()
        .to_string()
}
