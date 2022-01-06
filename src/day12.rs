use std::collections::HashMap;

use crate::selfprint::SelfPrint;

fn run_program<'a>(instructions: Vec<Vec<&'a str>>, registers: &mut HashMap<&'a str, i64>) {
    let mut index = 0i64;
    while let Ok(i) = usize::try_from(index) {
        if i >= instructions.len() {
            break;
        }
        let instruction = &instructions[i];
        match instruction[0] {
            "cpy" => {
                if let Ok(value) = instruction[1].parse::<i64>() {
                    registers.insert(instruction[2], value);
                } else {
                    registers.insert(instruction[2], *registers.get(instruction[1]).unwrap_or(&0));
                }
                index += 1;
            },
            "inc" => {
                registers.insert(instruction[1], registers.get(instruction[1]).unwrap_or(&0) + 1);
                index += 1;
            },
            "dec" => {
                registers.insert(instruction[1], registers.get(instruction[1]).unwrap_or(&0) - 1);
                index += 1;
            },
            "jnz" => { 
                if let Ok(value) = instruction[1].parse::<i64>() {
                    if value != 0 {
                        index += instruction[2].parse::<i64>().unwrap_or(0);
                    } else {
                        index += 1;
                    }
                } else {
                    if registers.get(instruction[1]).unwrap_or(&0) != &0 {
                        index += instruction[2].parse::<i64>().unwrap_or(0);
                    } else {
                        index += 1;
                    }
                }
            }
            _ => ()
        }
    }
}

pub fn part1(input: String) {
    let instructions = input
        .lines()
        .map(|l| l.split(" ").collect::<Vec<&str>>())
        .collect::<Vec<_>>();
    let mut registers: HashMap<&str, i64> = HashMap::new();
    run_program(instructions, &mut registers);
    registers.get("a").unwrap_or(&0).print();
}

pub fn part2(input: String) {
    let instructions = input
        .lines()
        .map(|l| l.split(" ").collect::<Vec<&str>>())
        .collect::<Vec<_>>();
    let mut registers: HashMap<&str, i64> = HashMap::new();
    registers.insert("c", 1);
    run_program(instructions, &mut registers);
    registers.get("a").unwrap_or(&0).print();
}
