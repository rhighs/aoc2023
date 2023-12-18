use itertools::Itertools;

type Image = Vec<Vec<char>>;
fn parse_input(input: &String) -> Image {
    input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec()
}

fn find_empty_rows(image: &Image) -> Vec<usize> {
    image
        .iter()
        .enumerate()
        .filter(|(_, row)| row.iter().all(|&v| v == '.'))
        .map(|(i, _)| i)
        .collect_vec()
}

fn find_empty_cols(image: &Image) -> Vec<usize> {
    (0..image[0].len())
        .filter(|&j| image.iter().all(|v| v[j] == '.'))
        .collect_vec()
}

fn find_galaxies_pos(image: &Image) -> Vec<(usize, usize)> {
    image
        .iter()
        .enumerate()
        .filter_map(|(i, v)| {
            let result = v
                .iter()
                .enumerate()
                .filter_map(|(j, &v)| if v == '#' { Some((i, j)) } else { None })
                .collect_vec();
            if result.len() > 0 {
                Some(result)
            } else {
                None
            }
        })
        .flat_map(|v| v.into_iter())
        .collect_vec()
}

fn distance(p1: (usize, usize), p2: (usize, usize)) -> usize {
    p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1)
}

fn expanded_distances(image: &Image, expansion_index: usize) -> Vec<usize> {
    let galaxies_pos = find_galaxies_pos(&image);
    let empty_rows = find_empty_rows(&image);
    let empty_cols = find_empty_cols(&image);
    let galaxies_pos = galaxies_pos
        .iter()
        .map(|v| {
            let rbehind = empty_rows.iter().filter(|&&r| v.0 > r).count();
            if rbehind > 0 {
                (v.0 + (rbehind * (expansion_index - 1)), v.1)
            } else {
                *v
            }
        })
        .map(|v| {
            let cbehind = empty_cols.iter().filter(|&&c| v.1 > c).count();
            if cbehind > 0 {
                (v.0, v.1 + (cbehind * (expansion_index - 1)))
            } else {
                v
            }
        })
        .collect_vec();
    galaxies_pos
        .iter()
        .enumerate()
        .flat_map(|(i, pos)| {
            (i + 1..galaxies_pos.len()).map(|j| distance(pos.clone(), galaxies_pos[j]))
        })
        .collect_vec()
}

pub fn p1(input: &String) -> String {
    let image = parse_input(input);
    let distances = expanded_distances(&image, 2);
    distances.iter().sum::<usize>().to_string()
}

pub fn p2(input: &String) -> String {
    let image = parse_input(input);
    let distances = expanded_distances(&image, 1000000);
    distances.iter().sum::<usize>().to_string()
}
