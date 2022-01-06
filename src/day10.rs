use std::{collections::{HashMap, HashSet}, cell::RefCell};

use regex::Regex;

use crate::selfprint::SelfPrint;

#[derive(Debug)]
enum Destination {
    Bot(u64),
    Output(u64)
}

#[derive(Debug)]
struct Bot {
    values: HashSet<u64>,
    low_destination: Option<Destination>,
    high_destination: Option<Destination>,
    is_done: bool
}

impl Bot {
    fn new() -> Bot {
        Bot { 
            values: HashSet::new(), 
            low_destination: None, 
            high_destination: None, 
            is_done: false 
        }
    }
}

fn parse_bots(input: String) -> Option<HashMap<u64, RefCell<Bot>>> {
    let mut bots = HashMap::new();
    let re = Regex::new(r"(?:value (\d+) goes to bot (\d+))|(?:bot (\d+) gives low to (bot|output) (\d+) and high to (bot|output) (\d+))").ok()?;

    for line in input.lines() {
        let captures = re.captures(line)?;
        if let (Some(value), Some(bot_number)) = (
            captures.get(1).map(|x| x.as_str().parse::<u64>().unwrap_or(0)),
            captures.get(2).map(|x| x.as_str().parse::<u64>().unwrap_or(0)),
        ) {
            let bot = bots.entry(bot_number).or_insert_with(|| RefCell::new(Bot::new()));
            bot.borrow_mut().values.insert(value);
        } else if let (
            Some(bot_number),
            Some(low_destination),
            Some(low_destination_number),
            Some(high_destination),
            Some(high_destination_number)
        ) = (
            captures.get(3).map(|x| x.as_str().parse::<u64>().unwrap_or(0)),
            captures.get(4).map(|x| x.as_str()),
            captures.get(5).map(|x| x.as_str().parse::<u64>().unwrap_or(0)),
            captures.get(6).map(|x| x.as_str()),
            captures.get(7).map(|x| x.as_str().parse::<u64>().unwrap_or(0))
        ) {
            let bot = bots.entry(bot_number).or_insert_with(|| RefCell::new(Bot::new()));
            bot.borrow_mut().low_destination = Some(
                match low_destination {
                    "bot" => Destination::Bot(low_destination_number),
                    _ => Destination::Output(low_destination_number)
                }
            );
            bot.borrow_mut().high_destination = Some(
                match high_destination {
                    "bot" => Destination::Bot(high_destination_number),
                    _ => Destination::Output(high_destination_number)
                }
            );
        }
    }

    Some(bots)
}

fn process_bots(input: String) -> Option<u64> {
    let bots = parse_bots(input)?;
    let mut outputs = HashMap::<u64, u64>::new();
    while bots.values().filter(|b| b.borrow().values.len() < 2).count() > 0 {
        for (&number, cell) in bots.iter().filter(|(_, b)| !b.borrow().is_done && b.borrow().values.len() == 2) {
            let mut bot = cell.borrow_mut();
            if bot.values.contains(&61) && bot.values.contains(&17) {
                println!("Part 1: {}", number);
            }
            let (low, high) = (
                bot.values.iter().min().unwrap_or(&0),
                bot.values.iter().max().unwrap_or(&0)
            );
            if let Some(Destination::Bot(low_destination)) = bot.low_destination {
                bots.get(&low_destination)?.borrow_mut().values.insert(*low);
            } else if let Some(Destination::Output(low_destination)) = bot.low_destination {
                outputs.insert(low_destination, *low);
            }
            if let Some(Destination::Bot(high_destination)) = bot.high_destination {
                bots.get(&high_destination)?.borrow_mut().values.insert(*high);
            } else if let Some(Destination::Output(high_destination)) = bot.high_destination {
                outputs.insert(high_destination, *high);
            }
            bot.is_done = true;
        }
    }
    Some(
        outputs.get(&0).unwrap_or(&0) * outputs.get(&1).unwrap_or(&0) * outputs.get(&2).unwrap_or(&0)
    )
}

pub fn part1(input: String) {
    process_bots(input).unwrap_or(0).print();
}
