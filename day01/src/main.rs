use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn part_1() -> u32 {
    let file = File::open("input.txt").expect("not found");
    let reader = BufReader::new(file);
    let mut left_list: Vec<u32> = vec![];
    let mut right_list: Vec<u32> = vec![];

    reader.lines().for_each(|line| {
        let str = line.unwrap();
        let l = str[0..=4].parse::<u32>().unwrap();
        let r = str[8..=12].parse::<u32>().unwrap();
        left_list.push(l);
        right_list.push(r);
    });
    left_list.sort();
    right_list.sort();
    left_list
        .iter()
        .zip(right_list.iter())
        .map(|(l, r)| l.abs_diff(*r))
        .sum()
}

fn part_2() -> u32 {
    let file = File::open("input.txt").expect("not found");
    let reader = BufReader::new(file);
    let mut left_list: Vec<u32> = vec![];
    let mut sim_score_map: HashMap<u32, u32> = HashMap::new();

    reader.lines().for_each(|line| {
        let str = line.unwrap();
        let l = str[0..=4].parse::<u32>().unwrap();
        let r = str[8..=12].parse::<u32>().unwrap();
        left_list.push(l);
        if let Some(count) = sim_score_map.get(&r) {
            sim_score_map.insert(r, count + 1);
        } else {
            sim_score_map.insert(r, 1);
        }
    });

    left_list
        .iter()
        .map(|f| {
            if let Some(count) = sim_score_map.get(f) {
                f * count
            } else {
                0
            }
        })
        .sum()
}

fn main() {
    println!("part1: {}", part_1());
    println!("part2: {}", part_2());
}
