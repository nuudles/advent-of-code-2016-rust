use std::collections::HashSet;

use itertools::iproduct;
use regex::Regex;

use crate::selfprint::SelfPrint;

pub fn part1(input: String) {
    let row_count = 6;
    let col_count = 50;

    let re = Regex::new(r"(rect|rotate) (?:(?:(\d+)x(\d+))|(?:(row|column) (?:x|y)=(\d+) by (\d+)))")
        .expect("Invalid Regex");

    let mut lights = HashSet::<(usize, usize)>::new();
    for line in input.lines() {
        let captures = re.captures(line).expect("Failed to get captures");
        match captures.get(1).unwrap().as_str() {
            "rotate" => {
                let is_row = captures.get(4).unwrap().as_str() == "row";
                let target = captures.get(5).unwrap().as_str().parse::<usize>().unwrap();
                let by = captures.get(6).unwrap().as_str().parse::<usize>().unwrap();
                let to_remove = lights
                    .iter()
                    .filter(|l| {
                        if is_row {
                            l.1 == target
                        } else {
                            l.0 == target
                        }
                    })
                    .copied()
                    .collect::<HashSet<_>>();
                let to_insert = to_remove
                    .iter()
                    .map(|l| {
                        if is_row {
                            ((l.0 + by) % col_count, l.1)
                        } else {
                            (l.0, (l.1 + by) % row_count)
                        }
                    })
                    .collect::<HashSet<_>>();
                lights = &lights - &to_remove;
                lights.extend(to_insert);
            },
            "rect" => {
                for (x, y) in iproduct!(
                    0..captures.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                    0..captures.get(3).unwrap().as_str().parse::<usize>().unwrap()
                ) {
                    lights.insert((x, y));
                }
            },
            _ => ()
        }
    }
    lights.len().print();
    for y in 0..row_count {
        let mut string = String::new();
        for x in 0..col_count {
            string.push(if lights.contains(&(x, y)) { '█' } else { ' '});
        }
        println!("{}", string);
    }
}
