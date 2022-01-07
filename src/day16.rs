use itertools::Itertools;

use crate::selfprint::SelfPrint;

fn checksum(input: String, length: usize) -> String {
    let mut string = input;
    while string.len() < length {
        let b = string.chars().rev().fold(String::new(), |mut result, c| {
            result.push(if c == '0' { '1' } else { '0' });
            result
        });
        string.push('0');
        string.push_str(b.as_str());
    }
    string.truncate(length);
    while string.len() % 2 == 0 {
        string = string.chars().chunks(2).into_iter().fold(String::new(), |mut result, mut chunk| {
            if let Some((a, b)) = chunk.next_tuple() {
                result.push(if a == b { '1' } else { '0' });
            }
            result
        });
    }
    string
}

pub fn part1(input: String) {
    checksum(input, 272).print();
}

pub fn part2(input: String) {
    checksum(input, 35651584).print();
}
