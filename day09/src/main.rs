use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone)]
enum Block {
    File { id: u64 },
    Free,
}

impl Block {
    fn print(&self) {
        match self {
            Block::File { id } => {
                print!("{}", id);
            }
            Block::Free => {
                print!(".");
            }
        }
    }
}

fn defrag(disk: &mut Vec<Block>) {
    for i in (0..disk.len()).rev() {
        let b1 = &disk[i];
        match b1 {
            Block::File { id: _ } => {
                for j in 0..disk.len() {
                    let b2 = &disk[j];
                    if match b2 {
                        Block::File { id: _ } => false,
                        Block::Free => true,
                    } {
                        disk.swap(i, j);
                        break;
                    }
                }
            }
            Block::Free => {}
        };
    }
    // Workaround to first element being free..
    disk.remove(0);
    disk.push(Block::Free);
}

fn checksum(disk: &mut Vec<Block>) -> u64 {
    disk.into_iter().enumerate().map(|(i, d)| match d {
        Block::File { id } => i as u64 * *id,
        Block::Free => 0,
    }).sum()
}

fn parse_disk(reader: BufReader<File>) -> Vec<Block> {
    let mut disk: Vec<Block> = Vec::new();
    let mut id = 0;
    reader
        .lines()
        .into_iter()
        .flat_map(|line| line.ok())
        .for_each(|line| {
            let mut iter = line.chars().into_iter().peekable();
            while iter.peek().is_some() {
                if let Some(file) = iter.next() {
                    for _ in 0..file.to_digit(10).unwrap() {
                        disk.push(Block::File { id: id });
                    }
                }

                if let Some(free) = iter.next() {
                    for _ in 0..free.to_digit(10).unwrap() {
                        disk.push(Block::Free);
                    }
                }

                id += 1;
            }
        });
    disk
}

fn part_1() -> u64 {
    let file = File::open("input.txt").expect("not found");
    let reader = BufReader::new(file);
    let mut disk = parse_disk(reader);
    defrag(&mut disk);
    checksum(&mut disk)
}

fn part_2() -> u64 {
    let file = File::open("input.txt").expect("not found");
    let reader = BufReader::new(file);
    0
}

fn main() {
    println!("part_1: {}", part_1());
    println!("part_2: {}", part_2());
}
