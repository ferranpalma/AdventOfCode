use std::collections::HashMap;

struct P2 {
    card_number: i32,
    matches: i32,
    repetitions: i32,
}

fn parse_to_vec(s: &str) -> Vec<i32> {
    let result: Vec<i32> = s
        .split_whitespace()
        .map(|n| n.parse::<i32>().unwrap())
        .collect();
    result
}

pub fn run(data: &str) {
    let mut winning_numbers: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut elf_numbers: HashMap<i32, HashMap<i32, i32>> = HashMap::new();

    // Part 1

    let game_matches: Vec<i32> = data
        .lines()
        .filter(|x| !x.is_empty())
        .map(|x| {
            let (game_number, game_cards) = x
                .trim_start_matches("Card")
                .trim_start_matches(char::is_whitespace)
                .split_once(':')
                .unwrap();
            let game_number = game_number.parse::<i32>().unwrap();

            let (winning_cards, elf_cards) = game_cards.split_once('|').unwrap();

            let winning_cards = parse_to_vec(winning_cards);
            let _ = winning_numbers.insert(game_number, winning_cards.clone());

            let elf_cards = parse_to_vec(elf_cards);

            let matches: i32 = winning_cards
                .iter()
                .map(|num| {
                    if elf_cards.iter().any(|elf_num| num == elf_num) {
                        1
                    } else {
                        0
                    }
                })
                .sum();

            matches
        })
        .collect();

    let result: u32 = game_matches
        .iter()
        .filter(|x| **x > 0)
        .map(|x| 2_u32.pow((x - 1) as u32))
        .sum();

    println!("{}", result);

    // Part 2

    let mut count = vec![1 as usize; data.lines().count()];

    for (index, matches) in game_matches.iter().enumerate() {
        let matches = *matches as usize;
        if matches == 0 {
            continue;
        }

        let instances = count[index];
        let new_instances = instances * matches;

        let final_card = if index + matches > count.len() {
            count.len() - 1
        } else {
            index + matches
        };

        for j in index + 1..=final_card {
            count[j] += instances;
        }
    }

    let result: usize = count.iter().sum();

    println!("Part 2: {}", result);
}
