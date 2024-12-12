use parse_display::{Display, FromStr};
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;
#[derive(Display, FromStr, PartialEq, Debug)]
enum Rule {
    #[display("{0}|{1}")]
    RuleBefore(u32, u32),
}
fn part_1() -> u32 {
    let file = File::open("input.txt").expect("not found");
    let reader = BufReader::new(file);
    let mut rulemap: HashMap<u32, HashSet<u32>> = HashMap::new();
    reader
        .lines()
        .into_iter()
        .map(|line| {
            if let Ok(line) = line {
                if let Ok(rule) = Rule::from_str(&line.as_str()) {
                    match rule {
                        Rule::RuleBefore(a, b) => {
                            let cond = rulemap.contains_key(&a);
                            if !cond {
                                rulemap.insert(a, HashSet::new());
                            }
                            if let Some(vec) = rulemap.get_mut(&a) {
                                vec.insert(b);
                            }
                        }
                    }
                    0
                } else {
                    let updates: Vec<u32> = line
                        .split(',')
                        .filter(|s| s.parse::<u32>().is_ok())
                        .map(|s| s.parse::<u32>().unwrap())
                        .collect();
                    let to_keep = updates.iter().enumerate().fold(true, |acc, (i, f)| {
                        if i < updates.len() - 1 {
                            if let Some(e) = rulemap.get(f) {
                                if e.contains(&updates[i + 1]) {
                                    acc && true
                                } else {
                                    acc && false
                                }
                            } else {
                                acc && false
                            }
                        } else {
                            // Do not check last
                            acc && true
                        }
                    });
                    if to_keep {
                        if let Some(u) = updates.get(updates.len() / 2) {
                            *u
                        } else {
                            0
                        }
                    } else {
                        0
                    }
                }
            } else {
                0
            }
        })
        .sum()
}

fn main() {
    println!("part1: {}", part_1());
}
