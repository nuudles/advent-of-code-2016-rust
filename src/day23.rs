use std::collections::{HashMap, HashSet};

use crate::selfprint::SelfPrint;

fn run_program<'a>(instructions: Vec<Vec<&'a str>>, registers: &mut HashMap<&'a str, i64>) {
    let mut index = 0i64;
    let mut toggled = HashSet::new();
    let valid_registers = ["a", "b", "c", "d"];
    while let Ok(i) = usize::try_from(index) {
        if i >= instructions.len() {
            break;
        }
        let instruction = &instructions[i];
        let command = if !toggled.contains(&i) {
            instruction[0]
        } else {
            match instruction[0] {
                "inc" => "dec",
                "dec" | "tgl" => "inc",
                "jnz" => "cpy",
                _ => "jnz"
            }
        };
        // println!("{} {:?} {:?}", command, instruction, registers);
        match command {
            "cpy" => {
                if !valid_registers.contains(&instruction[2]) {
                    index += 1;
                    continue;
                }
                if let Ok(value) = instruction[1].parse::<i64>() {
                    registers.insert(instruction[2], value);
                } else if valid_registers.contains(&instruction[1]) {
                    registers.insert(instruction[2], *registers.get(&instruction[1]).unwrap_or(&0));
                }
                index += 1;
            },
            "inc" => {
                if !valid_registers.contains(&instruction[1]) {
                    index += 1;
                    continue;
                }
                registers.insert(instruction[1], registers.get(instruction[1]).unwrap_or(&0) + 1);
                index += 1;
            },
            "dec" => {
                if !valid_registers.contains(&instruction[1]) {
                    index += 1;
                    continue;
                }
                registers.insert(instruction[1], registers.get(instruction[1]).unwrap_or(&0) - 1);
                index += 1;
            },
            "jnz" => { 
                let target = if valid_registers.contains(&instruction[2]) {
                    *registers.get(&instruction[2]).unwrap_or(&0)
                } else {
                    instruction[2].parse::<i64>().unwrap_or(0)
                };
                if let Ok(value) = instruction[1].parse::<i64>() {
                    if value != 0 {
                        index += target;
                    } else {
                        index += 1;
                    }
                } else {
                    if registers.get(instruction[1]).unwrap_or(&0) != &0 {
                        index += target;
                    } else {
                        index += 1;
                    }
                }
            }
            "tgl" => {
                toggled.insert((index + *registers.get(instruction[1]).unwrap_or(&0)) as usize);
                index += 1;
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
    registers.insert("a", 7);
    run_program(instructions, &mut registers);
    registers.get("a").unwrap_or(&0).print();
}

pub fn part2(input: String) {
    // I ran part 1 with various a initializations and looked for a pattern.
    // Based on the clues in the problem, I first guessed at exponents, but when that didn't
    // yield anything, I tried factorials and that helped me find the pattern, which is:
    // a! + (the numbers on lines 20 and 21 of the input multiplied together)
    let instructions = input
        .lines()
        .map(|l| l.split(" ").collect::<Vec<&str>>())
        .collect::<Vec<_>>();
    let offset = instructions[19][1].parse::<u64>().unwrap_or_default() * 
        instructions[20][1].parse::<u64>().unwrap_or_default();
    println!("{}", (1..=12).product::<u64>() + offset);
}
