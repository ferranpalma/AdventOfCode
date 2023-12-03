pub fn run(data: &str) {
    let part1_result: u32 = data
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let start = line
                .chars()
                .find(|&c| c >= '0' && c <= '9')
                .expect("Can't find number in String")
                .to_digit(10)
                .expect("Can't parse char to decimal value");
            let end = line
                .chars()
                .rfind(|&c| c >= '0' && c <= '9')
                .expect("Can't find number in String")
                .to_digit(10)
                .expect("Can't parse char to decimal value");
            start * 10 + end
        })
        .sum();

    let part2_result: u32 = data
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.to_string()
                .replace("one", "one1one")
                .replace("two", "two2two")
                .replace("three", "three3three")
                .replace("four", "four4four")
                .replace("five", "five5five")
                .replace("six", "six6six")
                .replace("seven", "seven7seven")
                .replace("eight", "eight8eight")
                .replace("nine", "nine9nine")
        })
        .map(|line| {
            let start = line
                .chars()
                .find(|&c| c >= '0' && c <= '9')
                .expect("Can't find number in String")
                .to_digit(10)
                .expect("Can't parse char to decimal value");
            let end = line
                .chars()
                .rfind(|&c| c >= '0' && c <= '9')
                .expect("Can't find number in String")
                .to_digit(10)
                .expect("Can't parse char to decimal value");
            start * 10 + end
        })
        .sum();

    println!("Part 1: {}; Part 2: {}", part1_result, part2_result);
}
