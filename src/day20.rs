use itertools::Itertools;

pub fn part1(input: String) {
    let mut lowest = 0;
    for (lower, upper) in input
        .lines()
        .map(|l| {
            let mut ranges = l.split("-").flat_map(|n| n.parse::<u64>().ok());
            let lower = ranges.next().unwrap_or_default();
            let upper = ranges.next().unwrap_or_default();
            (lower, upper)
        })
        .sorted_by_key(|r| r.0) {
            if lowest >= lower {
                lowest = upper + 1;
            } else {
                println!("{}", lowest);
                break
            }
        }
}

pub fn part2(input: String) {
    let mut lowest = 0;
    let mut count = 0;
    for (lower, upper) in input
        .lines()
        .map(|l| {
            let mut ranges = l.split("-").flat_map(|n| n.parse::<u64>().ok());
            let lower = ranges.next().unwrap_or_default();
            let upper = ranges.next().unwrap_or_default();
            (lower, upper)
        })
        .sorted_by_key(|r| r.0) {
            if lowest < lower {
                count += lower - lowest;
                lowest = upper + 1;
            } else if lowest < upper {
                lowest = upper + 1;
            }
        }
    count += 4294967296 - lowest;
    println!("{}", count);
}
