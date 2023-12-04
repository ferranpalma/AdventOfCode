#[derive(Debug)]
struct Game {
    id: u32,
    record: Vec<Color>,
    possible: bool,
}

#[derive(Debug)]
enum Color {
    Red(u32),
    Green(u32),
    Blue(u32),
}

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

pub fn run(data: &str) {
    let parsed_data: Vec<Game> = data
        .lines()
        .filter(|x| !x.is_empty())
        .map(|x| {
            let (game_id, game) = x.trim_start_matches("Game ").split_once(':').unwrap();
            let id = game_id.parse().unwrap();
            let mut record: Vec<Color> = Vec::new();
            let mut possible = true;
            for actions in game.split(';') {
                for action in actions.split(',') {
                    let (n, color) = action.trim_start_matches(" ").split_once(' ').unwrap();
                    let n = n.parse().unwrap();
                    let color = match color.as_bytes()[0] {
                        b'r' => {
                            if possible {
                                possible = n <= MAX_RED;
                            }
                            Color::Red(n)
                        }
                        b'g' => {
                            if possible {
                                possible = n <= MAX_GREEN;
                            }
                            Color::Green(n)
                        }
                        b'b' => {
                            if possible {
                                possible = n <= MAX_BLUE;
                            }
                            Color::Blue(n)
                        }
                        _ => panic!("Unknown color"),
                    };
                    record.push(color);
                }
            }
            Game {
                id,
                record,
                possible,
            }
        })
        .collect();

    let result: u32 = parsed_data
        .iter()
        .filter(|game| game.possible)
        .map(|game| game.id)
        .sum();

    println!("{:?}", result);
}
