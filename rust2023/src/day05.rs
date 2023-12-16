use std::{collections::HashMap, ops::RangeInclusive};

use range_ext::intersect::Intersect;

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
enum Category {
    Seed(i64),
    Soil(i64),
    Fertilizer(i64),
    Water(i64),
    Light(i64),
    Temperature(i64),
    Humidity(i64),
    Location(i64),
}

#[derive(Debug)]
struct MapInfo {
    dest: i64,
    source: i64,
    range: i64,
}

impl Category {
    fn new(value: i64) -> Self {
        Self::Seed(value)
    }

    pub fn destination(
        &self,
        source: i64,
        dest: i64,
        range: i64,
        found: &mut bool,
    ) -> Option<Self> {
        match self {
            Self::Seed(x) => {
                let end = source + range - 1;
                if *x >= source && *x <= end {
                    *found = true;
                    let offset = *x - source;
                    return Some(Self::Soil(dest + offset));
                }
                Some(Self::Soil(x.clone()))
            }
            Self::Soil(x) => {
                let end = source + range - 1;
                if *x >= source && *x <= end {
                    *found = true;
                    let offset = *x - source;
                    return Some(Self::Fertilizer(dest + offset));
                }
                Some(Self::Fertilizer(x.clone()))
            }
            Self::Fertilizer(x) => {
                let end = source + range - 1;
                if *x >= source && *x <= end {
                    *found = true;
                    let offset = *x - source;
                    return Some(Self::Water(dest + offset));
                }
                Some(Self::Water(x.clone()))
            }
            Self::Water(x) => {
                let end = source + range - 1;
                if *x >= source && *x <= end {
                    *found = true;
                    let offset = *x - source;
                    return Some(Self::Light(dest + offset));
                }
                Some(Self::Light(x.clone()))
            }
            Self::Light(x) => {
                let end = source + range - 1;
                if *x >= source && *x <= end {
                    *found = true;
                    let offset = *x - source;
                    return Some(Self::Temperature(dest + offset));
                }
                Some(Self::Temperature(x.clone()))
            }
            Self::Temperature(x) => {
                let end = source + range - 1;
                if *x >= source && *x <= end {
                    *found = true;
                    let offset = *x - source;
                    return Some(Self::Humidity(dest + offset));
                }
                Some(Self::Humidity(x.clone()))
            }
            Self::Humidity(x) => {
                let end = source + range - 1;
                if *x >= source && *x <= end {
                    *found = true;
                    let offset = *x - source;
                    return Some(Self::Location(dest + offset));
                }
                Some(Self::Location(x.clone()))
            }
            Self::Location(x) => Some(Self::Location(x.clone())),
        }
    }

    pub fn get_value(&self) -> i64 {
        match self {
            Category::Seed(value)
            | Category::Soil(value)
            | Category::Fertilizer(value)
            | Category::Water(value)
            | Category::Light(value)
            | Category::Temperature(value)
            | Category::Humidity(value)
            | Category::Location(value) => *value,
        }
    }

    pub fn get_key(&self) -> Category {
        match self {
            Category::Seed(_) => Category::Seed(0),
            Category::Soil(_) => Category::Soil(0),
            Category::Fertilizer(_) => Category::Fertilizer(0),
            Category::Water(_) => Category::Water(0),
            Category::Light(_) => Category::Light(0),
            Category::Temperature(_) => Category::Temperature(0),
            Category::Humidity(_) => Category::Humidity(0),
            Category::Location(_) => Category::Location(0),
        }
    }
}

fn get_category(category: &str) -> Category {
    match category {
        "seed-to-soil map:" => Category::Seed(0),
        "soil-to-fertilizer map:" => Category::Soil(0),
        "fertilizer-to-water map:" => Category::Fertilizer(0),
        "water-to-light map:" => Category::Water(0),
        "light-to-temperature map:" => Category::Light(0),
        "temperature-to-humidity map:" => Category::Temperature(0),
        "humidity-to-location map:" => Category::Humidity(0),
        _ => panic!("Does not match any category!"),
    }
}

fn mapped_info(data: &str) -> (Category, Vec<MapInfo>) {
    let mut iter = data.lines();
    let key = get_category(iter.next().expect("Can't read initial line"));
    let mut values: Vec<MapInfo> = Vec::new();
    for line in iter {
        let data: Vec<i64> = line
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<i64>().expect("Can't parse to int"))
            .collect();
        values.push(MapInfo {
            dest: data[0],
            source: data[1],
            range: data[2],
        })
    }

    (key, values)
}

fn remove_overlapping_ranges(ranges: Vec<RangeInclusive<i64>>) -> Vec<RangeInclusive<i64>> {
    let mut result: Vec<RangeInclusive<i64>> = ranges.clone();

    let mut i = 0;
    let mut j = 0;

    let mut overlaps = 0;

    while i < result.len() {
        j = i + 1;
        while j < result.len() {
            let mut r1 = result[i].clone();
            let mut r2 = result[j].clone();

            let r1_len = r1.end() - r1.start();
            let r2_len = r2.end() - r2.start();

            // r1 >= r2
            if r1_len < r2_len {
                let aux = r1.clone();
                r1 = r2.clone();
                r2 = aux.clone();
            }

            if r1.start() < r2.start() {
                // r1 and r2 never overlap
                if r1.end() < r2.start() {
                    j += 1;
                } else if r1.end() > r2.end() {
                    // r2 contained in r1
                    result.remove(j);
                    j += 1;
                } else {
                    // r2 start < r1 end < r2 start
                    let new_range = RangeInclusive::new(*r1.end(), *r2.start());
                    // insert new range
                    result[j] = new_range;
                    j += 1;
                }
            } else {
                // r1 and r2 never overlap
                if r2.end() < r1.start() {
                    j += 1;
                } else {
                    // no more cases, r1 is always bigger than r2
                    let new_range = RangeInclusive::new(*r1.start(), *r2.end());
                    // insert new range
                    result[j] = new_range;
                    j += 1;
                }
            }
        }
        i += 1;
    }
    result
}

pub fn run(data: &str) {
    let mut iter = data.lines();
    let mut iter_p2 = data.lines();

    let seeds_array: Vec<Category> = iter
        .next()
        .unwrap()
        .trim_start_matches("seeds: ")
        .split_whitespace()
        .map(|num| Category::new(num.parse().unwrap()))
        .collect();

    // Is not efficent but who cares :)
    let second_seeds_array: Vec<i64> = iter_p2
        .next()
        .unwrap()
        .trim_start_matches("seeds: ")
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();

    let mut seeds_range: Vec<RangeInclusive<i64>> = Vec::new();

    for index in (0..=second_seeds_array.len() - 1).step_by(2) {
        let start = second_seeds_array[index];
        let end = start + second_seeds_array[index + 1];
        seeds_range.push(RangeInclusive::new(start, end - 1))
    }

    let no_overlap_seeds = remove_overlapping_ranges(seeds_range);

    let mut map_info: HashMap<Category, Vec<MapInfo>> = HashMap::new();
    let map_data: Vec<&str> = data.split("\n\n").skip(1).collect();

    for line in map_data {
        let (value, key) = mapped_info(line);
        map_info.insert(value, key);
    }

    // let mut location_values: Vec<i64> = vec![0; seeds_array.len()];
    // for (index, seed) in seeds_array.iter().enumerate() {
    //     let mut origin = seed.clone();
    //     let mut dest = seed.clone();
    //     loop {
    //         let mut found = false;
    //         let info = map_info
    //             .get(&origin.get_key())
    //             .expect("No value for key {origin}");
    //         for values in info {
    //             dest = origin
    //                 .destination(values.source, values.dest, values.range, &mut found)
    //                 .expect("No destination for {origin}");
    //             if found {
    //                 break;
    //             }
    //         }
    //
    //         location_values[index] = dest.get_value();
    //
    //         match dest {
    //             Category::Location(_) => {
    //                 break;
    //             }
    //             _ => {
    //                 origin = dest;
    //                 continue;
    //             }
    //         }
    //     }
    // }
    //
    // let result = location_values.iter().min().expect("Can't find min value.");

    // Part 2

    let mut seeds_p2: Vec<Category> = Vec::new();

    for range in no_overlap_seeds {
        for seed in range {
            seeds_p2.push(Category::new(seed));
        }
    }
    println!("Start processing");
    let mut location_values: Vec<i64> = vec![0; seeds_p2.len()];
    for (index, seed) in seeds_p2.iter().enumerate() {
        let mut origin = *seed;
        let mut dest = *seed;
        loop {
            let mut found = false;
            let info = map_info
                .get(&origin.get_key())
                .expect("No value for key {origin}");
            for values in info {
                dest = origin
                    .destination(values.source, values.dest, values.range, &mut found)
                    .expect("No destination for {origin}");
                if found {
                    break;
                }
            }

            match dest {
                Category::Location(_) => {
                    location_values[index] = dest.get_value();
                    break;
                }
                _ => {
                    origin = dest;
                    continue;
                }
            }
        }
    }

    println!("End processing");

    let result = location_values.iter().min().expect("Can't find min value.");

    println!("Result: {:?}", result);
}
