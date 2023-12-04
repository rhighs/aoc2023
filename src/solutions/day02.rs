#[derive(Debug)]
enum Cube {
    Red(i32),
    Green(i32),
    Blue(i32),
}

#[derive(Debug)]
struct Game {
    id: i32,
    rounds: Vec<Vec<Cube>>
}

fn parse_input(input: &String) -> Vec<Game> {
    let mut result = Vec::new();
    for line in input.lines() {
        let gameid2data = line
            .split(":")
            .into_iter()
            .map(|piece| {
                if piece.contains("Game") {
                    piece.split(" ").collect::<Vec<&str>>()[1].into()
                } else {
                    String::from(piece)
                }
            })
            .map(|v| v.trim().into())
            .collect::<Vec<String>>();
        let id: i32 = gameid2data[0].parse().unwrap();
        let rounds: Vec<Vec<Cube>> = gameid2data[1]
            .split(";")
            .map(|r| {
                r.split(",")
                    .map(|v| v.trim().into())
                    .collect::<Vec<String>>()
            })
            .map(|hand| {
                hand.iter().fold(
                    Vec::<Cube>::new(),
                    |mut value: Vec<Cube>, single_draw: &String| {
                        let values = single_draw
                            .split(" ")
                            .map(|v| v.into())
                            .collect::<Vec<String>>();
                        let result = match values[1].as_str() {
                            "red" => Cube::Red(values[0].parse().unwrap()),
                            "green" => Cube::Green(values[0].parse().unwrap()),
                            "blue" => Cube::Blue(values[0].parse().unwrap()),
                            _ => unreachable!(),
                        };
                        value.push(result);
                        value
                    },
                )
            })
            .collect();
        result.push(Game { id, rounds })
    }
    result
}

const PRESET_NO_RED: i32 = 12;
const PRESET_NO_GREEN: i32 = 13;
const PRESET_NO_BLUE: i32 = 14;

struct GameState {
    red: i32,
    green: i32,
    blue: i32,
}

pub fn p1(input: &String) -> String {
    let game_data = parse_input(input);
    let mut possible: Vec<i32> = vec![];

    for game in game_data {
        let mut ok = true;
        for round in game.rounds {
            let mut game_state = GameState { red: PRESET_NO_RED, green: PRESET_NO_GREEN, blue: PRESET_NO_BLUE };
            for hand in round {
                match hand {
                    Cube::Red(v) => game_state.red -= v,
                    Cube::Green(v) => game_state.green -= v,
                    Cube::Blue(v) => game_state.blue -= v,
                }
            }
            if game_state.red < 0 || game_state.green < 0 || game_state.blue < 0 {
                ok = false;
            }
        }
        if ok {
            possible.push(game.id);
        }
    }

    possible.into_iter().sum::<i32>().to_string()
}

pub fn p2(input: &String) -> String {
    let game_data = parse_input(input);

    let mut result: i32 = 0;
    for game in game_data {
        let mut game_state = GameState { red: 0, green: 0, blue: 0 };
        for round in game.rounds {
            for hand in round {
                match hand {
                    Cube::Red(v) => game_state.red = game_state.red.max(v),
                    Cube::Green(v) => game_state.green = game_state.green.max(v),
                    Cube::Blue(v) => game_state.blue = game_state.blue.max(v),
                }
            }
        }
        result += game_state.red * game_state.green * game_state.blue;
    }

    result.to_string()
}