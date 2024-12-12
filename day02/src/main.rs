use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
#[derive(PartialEq, Clone, Copy)]
enum LevelChange {
    INCREASE,
    DECREASE,
    NOCREASE,
    UNDEFINED,
}
fn part_1() -> u32 {
    let file = File::open("input.txt").expect("not found");
    let reader = BufReader::new(file);
    reader.lines().fold(0, |acc, line| {
        let mut level = line
            .as_ref()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .peekable();
        let mut prev_level_change: LevelChange = LevelChange::UNDEFINED;
        while let (Some(x), Some(y)) = (level.next(), level.peek()) {
            let curr_level_change = if x < *y {
                LevelChange::INCREASE
            } else if x > *y {
                LevelChange::DECREASE
            } else {
                LevelChange::NOCREASE
            };
            if prev_level_change != LevelChange::UNDEFINED && curr_level_change != prev_level_change
            {
                return acc;
            }
            if x.abs_diff(*y) > 3 {
                return acc;
            }
            prev_level_change = curr_level_change;
        }
        acc + 1
    })
}

fn main() {
    println!("part1: {}", part_1());
}
