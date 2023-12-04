#[derive(Debug)]
struct Game {
    id: u32,
    record: Vec<Color>,
    possible: bool,
    minimums: (u32, u32, u32),
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
            let mut minimums = (0, 0, 0);
            for actions in game.split(';') {
                for action in actions.split(',') {
                    let (n, color) = action.trim_start_matches(" ").split_once(' ').unwrap();
                    let n = n.parse().unwrap();
                    let color = match color.as_bytes()[0] {
                        b'r' => {
                            if possible {
                                possible = n <= MAX_RED;
                            }
                            minimums.0 = minimums.0.max(n);
                            Color::Red(n)
                        }
                        b'g' => {
                            if possible {
                                possible = n <= MAX_GREEN;
                            }
                            minimums.1 = minimums.1.max(n);
                            Color::Green(n)
                        }
                        b'b' => {
                            if possible {
                                possible = n <= MAX_BLUE;
                            }
                            minimums.2 = minimums.2.max(n);
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
                minimums,
            }
        })
        .collect();

    let result: u32 = parsed_data
        .iter()
        .filter(|game| game.possible)
        .map(|game| game.id)
        .sum();
    let result_p2: u32 = parsed_data
        .iter()
        .map(|game| game.minimums.0 * game.minimums.1 * game.minimums.2)
        .sum();

    println!("{:?}", result_p2);
}
