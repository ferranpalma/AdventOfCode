use regex::Regex;

struct Point(u32, u32);
pub fn run(data: &str) {
    let rows = data.lines().count() as u32;
    let cols = data.lines().next().unwrap().chars().count() as u32;

    let mut numbers: Vec<(&str, Point)> = vec![];
    let number_regex = Regex::new(r"\d+").unwrap();

    for (row, line) in data.lines().enumerate() {
        for number in number_regex.find_iter(line) {
            numbers.push((number.as_str(), Point(row as u32, number.start() as u32)));
        }
    }

    let mut symbols: Vec<(char, Point)> = vec![];
    for (row, line) in data.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if !ch.is_ascii_digit() && ch != '.' {
                symbols.push((ch, Point(row as u32, col as u32)));
            }
        }
    }

    // Part 1

    let mut part_numbers: Vec<u32> = vec![];
    for (num_str, num_start_pos) in numbers.iter() {
        let num_end_pos = Point(num_start_pos.0, num_start_pos.1 + num_str.len() as u32 - 1);
        for (_, symbol_pos) in symbols.iter() {
            let neighbors = [
                // Three elements up the actual position
                Point(symbol_pos.0 - 1, symbol_pos.1 - 1),
                Point(symbol_pos.0 - 1, symbol_pos.1),
                Point(symbol_pos.0 - 1, symbol_pos.1 + 1),
                // Left and right elements
                Point(symbol_pos.0, symbol_pos.1 - 1),
                Point(symbol_pos.0, symbol_pos.1 + 1),
                // Three elements down the actual position
                Point(symbol_pos.0 + 1, symbol_pos.1 - 1),
                Point(symbol_pos.0 + 1, symbol_pos.1),
                Point(symbol_pos.0 + 1, symbol_pos.1 + 1),
            ]
            .into_iter()
            .filter(|p| p.0 >= 0 && p.1 >= 0 && p.0 < rows as u32 && p.1 < cols as u32)
            .collect::<Vec<Point>>();

            for neighbor in neighbors.iter() {
                if neighbor.0 == num_start_pos.0
                    && neighbor.1 >= num_start_pos.1
                    && neighbor.1 <= num_end_pos.1
                {
                    part_numbers.push(num_str.parse::<u32>().unwrap());
                    break;
                }
            }
        }
    }

    let result: u32 = part_numbers.iter().sum();

    println!("Result: {}", result);

    // Part 2

    let mut symbols: Vec<(char, Point)> = vec![];
    for (row, line) in data.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == '*' {
                symbols.push((ch, Point(row as u32, col as u32)));
            }
        }
    }

    let mut gear_ratios: Vec<u32> = vec![];
    for (_, gear_pos) in symbols.iter() {
        let neighbors = [
            Point(gear_pos.0 - 1, gear_pos.1 - 1),
            Point(gear_pos.0 - 1, gear_pos.1),
            Point(gear_pos.0 - 1, gear_pos.1 + 1),
            Point(gear_pos.0, gear_pos.1 - 1),
            Point(gear_pos.0, gear_pos.1 + 1),
            Point(gear_pos.0 + 1, gear_pos.1 - 1),
            Point(gear_pos.0 + 1, gear_pos.1),
            Point(gear_pos.0 + 1, gear_pos.1 + 1),
        ]
        .into_iter()
        .filter(|p| p.0 >= 0 && p.1 >= 0 && p.0 < rows as u32 && p.1 < cols as u32)
        .collect::<Vec<Point>>();
        let mut adjacent_nums: Vec<u32> = vec![];
        for (num_str, num_start_pos) in numbers.iter() {
            let num_end_pos = Point(num_start_pos.0, num_start_pos.1 + num_str.len() as u32 - 1);
            for neighbor in neighbors.iter() {
                if neighbor.0 == num_start_pos.0
                    && neighbor.1 >= num_start_pos.1
                    && neighbor.1 <= num_end_pos.1
                {
                    adjacent_nums.push(num_str.parse::<u32>().unwrap());
                    break;
                }
            }
        }
        if adjacent_nums.len() == 2 {
            gear_ratios.push(adjacent_nums.iter().product());
        }
    }

    let result: u32 = gear_ratios.iter().sum();
    println!("Result: {}", result);
}
