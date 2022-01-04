use std::collections::HashMap;
use std::str;

use itertools::Itertools;
use md5;

pub fn part1(input: String) {
    let mut password = String::new();
    let mut index = 0;
    while password.len() < 8 {
        let mut hash = input.clone();
        hash.push_str(&index.to_string());
        let digest = md5::compute(&hash);
        if digest[0] == 0x00 && digest[1] == 0x00 && (digest[2] & 0xF0) == 0x00 {
            println!("{}: {:?}", hash, digest);
            let c = digest[2] & 0x0F;
            password.push(
                if c < 10 {
                    b'0' + c
                } else {
                    b'a' + c - 10
                } as char
            );
        }
        index += 1;
    }
    println!("{}", password);
}

pub fn part2(input: String) {
    let mut password_map = HashMap::new();
    let mut index = 1739529; // First interesting index based on part 1
    while password_map.len() < 8 {
        let mut hash = input.clone();
        hash.push_str(&index.to_string());
        let digest = md5::compute(&hash);
        if digest[0] == 0x00 && digest[1] == 0x00 && (digest[2] & 0xF0) == 0x00 {
            let i = digest[2] & 0x0F;
            if i < 8 && !password_map.contains_key(&i) {
                password_map.insert(i, digest[3] >> 4);
            }
        }
        index += 1;
    }
    let password = password_map
        .iter()
        .sorted_by_key(|(&k, _)| k)
        .map(|(_, &c)|
            if c < 10 {
                b'0' + c
            } else {
                b'a' + c - 10
            }
        )
        .collect::<Vec<_>>();
    println!("{}", str::from_utf8(&password).unwrap_or(""));
}
