use std::{num::NonZeroUsize, ops::Range};

#[derive(Debug)]
struct Map {
    src_dest_map: Vec<(Range<usize>, Range<usize>)>,
}

impl Map {
    fn from_categories(mut src_dest_map: Vec<(Range<usize>, Range<usize>)>) -> Self {
        // since none of the categories are overlapping, they will NOT have the same start,
        // so sorting by start is fine.

        src_dest_map.sort_by_key(|(src, _)| src.start);

        Self { src_dest_map }
    }

    fn get(&self, key: usize) -> usize {
        // binary search the key's range first.
        let ind = self
            .src_dest_map
            .partition_point(|(src, _)| src.start <= key)
            - 1;

        self.src_dest_map
            .get(ind)
            .and_then(|(src, dest)| (key <= src.end).then_some(dest.start + (key - src.start)))
            .unwrap_or(key)
    }
}

fn parse_maps(s: &str) -> Vec<Map> {
    s.trim()
        .split("\n\n")
        .map(|map_str| {
            Map::from_categories(
                map_str
                    .lines()
                    .skip(1)
                    .map(|src_dest_mapping| {
                        let src_dest_mapping = src_dest_mapping.split_whitespace();
                        let src_dest_n = src_dest_mapping.take(3);

                        let src_dest_n = src_dest_n.map(|x| x.parse::<usize>().unwrap());

                        let [dest, src, n]: [usize; 3] =
                            src_dest_n.collect::<Vec<usize>>().try_into().unwrap();

                        (src..src + n, dest..dest + n)
                    })
                    .collect(),
            )
        })
        .collect()
}

pub fn level1(s: &str) -> usize {
    let (seeds, rest) = s.split_once("\n\n").unwrap();
    let seeds = seeds
        .strip_prefix("seeds: ")
        .unwrap()
        .split_whitespace()
        .map(|seed| seed.parse::<usize>().unwrap());

    let maps: Vec<Map> = parse_maps(rest);

    seeds
        .inspect(|seed| println!("Working for {seed}..."))
        .map(|seed| maps.iter().fold(seed, |curr_seed, map| map.get(curr_seed)))
        .inspect(|location| println!("We got the location {location}.\n"))
        .min()
        .unwrap()
}

pub fn level2(s: &str) -> usize {
    let (seeds, rest) = s.split_once("\n\n").unwrap();
    let seeds: Vec<usize> = seeds
        .strip_prefix("seeds:")
        .unwrap()
        .split_whitespace()
        .map(|seed| seed.parse::<usize>().unwrap())
        .collect();

    let maps = parse_maps(rest);

    // Time to break out threads babyyyy, independent work, easily done parallely 😌
    // I could use rayon, but first I wanna do it myself.

    let work = seeds
        .chunks_exact(2)
        .map(|range| {
            let (len, range_start) = (range[1], range[0]);
            let seeds = range_start..range_start + len;
            seeds.map(|seed| maps.iter().fold(seed, |curr_seed, map| map.get(curr_seed)))
        })
        .collect();

    println!("{work:?}");

    do_it_in_parallel(work)
}

fn do_it_in_parallel(works: Vec<impl Iterator<Item = usize> + Send>) -> usize {
    use std::thread;

    thread::scope(|scope| {
        // VERY IMP! Tricky tricky bug!
        // If you think you're being smart by not collecting the join handles,
        // (i.e saving memory on a vector), you're doing an even bigger sin!
        // Rust iterators are lazy, meaning without .collect(), .map(|_| {..})
        // isn't even spawning the thread!
        // After that, when each join_handle is joined, THEN, the thread is actually spawned,
        // and does its work, one-by-one :(

        let join_handles: Vec<thread::ScopedJoinHandle<usize>> = works
            .into_iter()
            .map(|work| scope.spawn(|| work.min().unwrap()))
            .collect();

        join_handles
            .into_iter()
            .map(|join_handle| join_handle.join().unwrap())
            .min()
            .unwrap()
    })
}
