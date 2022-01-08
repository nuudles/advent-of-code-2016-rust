use std::collections::{BTreeMap, HashSet};

use itertools::Itertools;
use pathfinding::prelude::bfs;
use regex::Regex;

use crate::selfprint::SelfPrint;

fn parse_nodes(input: &String) -> BTreeMap<(i64, i64), (u64, u64, u64, u64)> {
    let re = Regex::new(
        r"node-x(\d+)-y(\d+)\s+(\d+)T\s+(\d+)T\s+(\d+)T\s+(\d+)%"
    ).expect("Could not compile Regex");
    input
        .lines()
        .fold(BTreeMap::new(), |mut map, l| {
            if let Some(captures) = re.captures(l) {
                if let (
                    Some(x),
                    Some(y),
                    Some(size),
                    Some(used),
                    Some(avail),
                    Some(use_percent)
                ) = (
                    captures.get(1).map(|m| m.as_str().parse::<i64>().unwrap_or_default()),
                    captures.get(2).map(|m| m.as_str().parse::<i64>().unwrap_or_default()),
                    captures.get(3).map(|m| m.as_str().parse::<u64>().unwrap_or_default()),
                    captures.get(4).map(|m| m.as_str().parse::<u64>().unwrap_or_default()),
                    captures.get(5).map(|m| m.as_str().parse::<u64>().unwrap_or_default()),
                    captures.get(6).map(|m| m.as_str().parse::<u64>().unwrap_or_default())
                ) {
                    map.insert((x, y), (size, used, avail, use_percent));
                }
            }
            map
        })
}

pub fn part1(input: String) {
    parse_nodes(&input)
        .values()
        .tuple_combinations()
        .filter(|(a, b)| {
            a.3 != 0 && a.1 <= b.2 || b.3 != 0 && b.1 <= a.2
        })
        .count()
        .print();
}

pub fn part2(input: String) {
    let nodes = parse_nodes(&input);
    let blockers = nodes
        .iter()
        .filter(|(_, n)| n.1 > 92)
        .map(|(pos, _)| *pos)
        .collect::<HashSet<_>>();
    let free_pos = nodes
        .iter()
        .filter(|(_, n)| n.1 == 0)
        .nth(0)
        .unwrap()
        .0;
    let path = bfs(
        &((30i64, 0i64), *free_pos),
        |(goal, free_pos)| {
            let mut successors = vec![];
            for (x, y) in [
                (free_pos.0 + 1, free_pos.1),
                (free_pos.0 - 1, free_pos.1),
                (free_pos.0, free_pos.1 - 1),
                (free_pos.0, free_pos.1 + 1)
            ] {
                if !(0..=30).contains(&x) || !(0..=30).contains(&y) || blockers.contains(&(x, y)) {
                    continue;
                }
                if goal == &(x, y) {
                    successors.push((*free_pos, *goal));
                } else {
                    successors.push((*goal, (x, y)));
                }
            }
            successors
        },
        |(goal, _)| goal == &(0, 0)
    );
    println!("{}", path.unwrap().len() - 1);
}
