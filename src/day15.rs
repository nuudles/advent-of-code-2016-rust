use regex::Regex;

use crate::selfprint::SelfPrint;

fn time_to_press(input: String, hard_mode: bool) -> Option<u64> {
    let re = Regex::new(r"Disc #(\d+) has (\d+) positions; at time=0, it is at position (\d+).").ok()?;
    let mut discs = input
        .lines()
        .flat_map(|l| {
            re.captures(l).and_then(|c| {
                if let (Some(disc), Some(size), Some(position)) = (
                    c.get(1).map(|x| x.as_str().parse::<u64>().ok().unwrap_or(0)),
                    c.get(2).map(|x| x.as_str().parse::<u64>().ok().unwrap_or(0)),
                    c.get(3).map(|x| x.as_str().parse::<u64>().ok().unwrap_or(0))
                ) {
                    Some((disc, size, position))
                } else {
                    None
                }
            })
        })
        .collect::<Vec<_>>();
    if hard_mode {
        discs.push((discs.len() as u64 + 1, 11, 0));
    }

    let mut time = 0;
    while discs.iter().map(|(disc, size, position)| (time + position + disc) % size).sum::<u64>() != 0 {
        time += 1;
    }
    Some(time)
}

pub fn part1(input: String) {
    time_to_press(input, false).unwrap_or(0).print();
}

pub fn part2(input: String) {
    time_to_press(input, true).unwrap_or(0).print();
}
