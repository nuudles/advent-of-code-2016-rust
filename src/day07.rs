use std::collections::HashSet;

use itertools::Itertools;

use crate::selfprint::SelfPrint;

fn supports_tls(line: &&str) -> bool {
    let mut abba_seen = false;
    for (i, s) in line.split(&['[', ']'][..]).enumerate() {
        if abba_seen && i % 2 == 0 {
            continue;
        }
        for (a, b, c, d) in s.chars().tuple_windows() {
            if a == d && b == c && a != b {
                if i % 2 == 0 {
                    abba_seen = true;
                } else {
                    return false;
                }
            }
        }
    }
    abba_seen
}

fn supports_ssl(line: &&str) -> bool {
    let mut abas = HashSet::<String>::new();
    let mut babs = HashSet::<String>::new();
    for (i, s) in line.split(&['[', ']'][..]).enumerate() {
        for (a, b, c) in s.chars().tuple_windows() {
            if a == c && a != b {
                if i % 2 == 0 {
                    let aba = vec![a, b, a].iter().collect::<String>();
                    abas.insert(aba);
                } else {
                    let bab = vec![b, a, b].iter().collect::<String>();
                    babs.insert(bab);
                }
            }
        }
    }
    abas.intersection(&babs).count() > 0
}

pub fn part1(input: String) {
    input
        .lines()
        .filter(supports_tls)
        .count()
        .print();
}

pub fn part2(input: String) {
    input
        .lines()
        .filter(supports_ssl)
        .count()
        .print();
}
