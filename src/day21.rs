use itertools::Itertools;

use crate::selfprint::SelfPrint;

fn run_instructions(lines: &Vec<&str>, password: &str, unscramble: bool) -> String {
    lines
        .iter()
        .fold(password.chars().collect_vec(), |mut chars, l| {
            let instruction = l.split(" ").collect_vec();
            match instruction[0] {
                "swap" => {
                    let x: usize;
                    let y: usize;
                    if instruction[1] == "position" {
                        x = instruction[2].parse::<usize>().unwrap_or_default();
                        y = instruction[5].parse::<usize>().unwrap_or_default();
                    } else {
                        let a = instruction[2].chars().nth(0).unwrap_or_default();
                        let b = instruction[5].chars().nth(0).unwrap_or_default();
                        x = chars.iter().position(|c| c == &a).unwrap_or_default();
                        y = chars.iter().position(|c| c == &b).unwrap_or_default();
                    }
                    let c = chars[x];
                    chars[x] = chars[y];
                    chars[y] = c;
                },
                "rotate" => {
                    if instruction[1] == "based" {
                        let a = instruction[6].chars().nth(0).unwrap_or_default();
                        let mut x = chars.iter().position(|c| c == &a).unwrap_or_default();
                        if !unscramble {
                            if x >= 4 {
                                x += 1;
                            }
                            x = (x + 1) % chars.len();
                            chars.rotate_right(x);
                        } else {
                            let mut count = 0;
                            while (x + 1 + if x >= 4 { 1 } else { 0 }) % chars.len() != count {
                                chars.rotate_left(1);
                                count += 1;
                                if x == 0 {
                                    x = chars.len() - 1;
                                } else {
                                    x -= 1;
                                }
                            }
                        }
                    } else {
                        let steps = instruction[2].parse::<usize>().unwrap_or_default();
                        if instruction[1] == "left" && !unscramble || instruction[1] == "right" && unscramble {
                            chars.rotate_left(steps);
                        } else {
                            chars.rotate_right(steps);
                        }
                    }
                },
                "reverse" => {
                    let lower = instruction[2].parse::<usize>().unwrap_or_default();
                    let upper = instruction[4].parse::<usize>().unwrap_or_default();
                    let reversed = chars[lower..=upper].iter().rev().map(|c| *c).collect_vec();
                    for i in lower..=upper {
                        chars[i] = reversed[i - lower];
                    }
                },
                "move" => {
                    let from: usize;
                    let to: usize;
                    if !unscramble {
                        from = instruction[2].parse::<usize>().unwrap_or_default();
                        to = instruction[5].parse::<usize>().unwrap_or_default();
                    } else {
                        from = instruction[5].parse::<usize>().unwrap_or_default();
                        to = instruction[2].parse::<usize>().unwrap_or_default();
                    }
                    let c = chars.remove(from);
                    chars.insert(to, c);
                },
                _ => ()
            }
            chars
        })
        .iter()
        .collect()
}

pub fn part1(input: String) {
    run_instructions(&input.lines().collect(), "abcdefgh", false).print();
}

pub fn part2(input: String) {
    run_instructions(&input.lines().rev().collect(), "fbgdceah", true).print();
}
