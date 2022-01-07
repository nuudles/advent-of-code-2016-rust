use std::collections::HashSet;

use itertools::Itertools;

use crate::selfprint::SelfPrint;

fn find_pads(input: String, should_stretch: bool) {
    let mut index = 0;

    let mut pad_indices = HashSet::<usize>::new();
    let mut potential_pads = HashSet::<(char, usize)>::new();

    while pad_indices.len() < 64 {
        potential_pads.retain(|(_, i)| index < i + 1000);

        let mut hash = input.clone();
        hash.push_str(&index.to_string());
        let digest = if !should_stretch {
            format!("{:x}", md5::compute(&hash))
        } else {
            (0..2016).fold(format!("{:x}", md5::compute(&hash)), |h, _| format!("{:x}", md5::compute(&h)))
        };

        for (a, b, c, d, e) in digest.chars().tuple_windows() {
            if a == b && b == c && c == d && d == e {
                for (_, i) in potential_pads.iter().filter(|(ch, _)| ch == &a) {
                    pad_indices.insert(*i);
                }
                potential_pads.retain(|(ch, _)| ch != &a);
            }
        }
        for (a, b, c) in digest.chars().tuple_windows() {
            if a == b && b == c {
                potential_pads.insert((a, index));
                break;
            }
        }
        index += 1;
    }

    pad_indices.iter().sorted().nth(63).unwrap_or(&0).print();
}

pub fn part1(input: String) {
    find_pads(input, false);
}

pub fn part2(input: String) {
    find_pads(input, true);
}
