use counter::Counter;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::str;

use crate::selfprint::SelfPrint;

fn real_room_sector_id(line: &str) -> Option<u64> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"([a-z\-]+)(\d+)\[([a-z]+)\]").unwrap();
    }

    let captures = RE.captures(line)?;
    let most_common_five = captures
        .get(1)?
        .as_str()
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect::<Counter<_>>()
        .iter()
        .sorted_by(|a, b| {
            if a.1 == b.1 {
                return a.0.cmp(b.0);
            }
            b.1.cmp(a.1)
        })
        .take(5)
        .map(|c| c.0)
        .collect::<String>();

    if captures.get(3)?.as_str() == most_common_five {
        let sector_id = captures.get(2)?.as_str().parse().ok()?;
        let name = captures
            .get(1)?
            .as_str()
            .bytes()
            .map(|b| {
                if b == b'-' {
                    b' '
                } else {
                    b'a' + (((b - b'a') as u64 + sector_id) % 26) as u8
                }
            })
            .collect::<Vec<_>>();
        println!("{}: {}", sector_id, str::from_utf8(&name).unwrap_or(""));
        Some(sector_id)
    } else {
        None
    }
}

pub fn part1(input: String) {
    input
        .lines()
        .filter_map(|l| real_room_sector_id(l))
        .sum::<u64>()
        .print();
}
