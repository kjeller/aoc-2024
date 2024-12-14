use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Mul,
    Concat,
}

impl Operator {
    fn operate(self, l: u64, r: u64) -> u64 {
        match self {
            Operator::Add => l + r,
            Operator::Mul => l * r,
            Operator::Concat => format!("{}{}", l, r).parse().unwrap(),
        }
    }
}

fn generate_permutations(length: usize, enable_concat: bool) -> Vec<Vec<Operator>> {
    let mut permutations = vec![vec![]];

    for _ in 0..length {
        let mut new_permutations = vec![];
        for perm in &permutations {
            let mut with_add = perm.clone();
            with_add.push(Operator::Add);
            new_permutations.push(with_add);

            let mut with_mul = perm.clone();
            with_mul.push(Operator::Mul);
            new_permutations.push(with_mul);

            if enable_concat {
            let mut with_concat= perm.clone();
            with_concat.push(Operator::Concat);
            new_permutations.push(with_concat);
            }
        }
        permutations = new_permutations;
    }

    permutations
}


fn parse(reader: BufReader<File> ) -> Vec<(u64, Vec<u64>)>  {
    reader
        .lines()
        .into_iter()
        .flat_map(|line| line.ok())
        .map(|line| {
            let mut spliter = line.split(':');
            let result: u64 = spliter.next().unwrap().parse::<u64>().unwrap();
            let rest: Vec<u64> = spliter
                .next()
                .unwrap()
                .split(' ')
                .skip(1)
                .flat_map(|x| x.parse::<u64>().ok())
                .collect();
            (result, rest)
        })
        .collect()
}

fn evaluate(input: Vec<(u64, Vec<u64>)>, enable_concat: bool) -> u64 {
    let mut total_calibration_result = 0;
    input.iter().for_each(|(result, expr)| {
        // Optimize by generating permutation tables at compile time
        let mut permutations: Vec<_> = generate_permutations(expr.len(), enable_concat);
        while let Some(op) = permutations.pop() {
            let total = expr.iter().enumerate().fold(0, |l, (i, r)| {
                if i == 0 {
                    // continue
                    *r
                } else {
                    let res = op[i - 1].operate(l, *r);
                    res
                }
            });
            if total == *result {
            total_calibration_result += total;
                break;
            }
        }
    });
    total_calibration_result
}

fn part_1() -> u64 {
    let file = File::open("input.txt").expect("not found");
    let reader = BufReader::new(file);
    evaluate(parse(reader), false)
}

fn part_2() -> u64 {
    let file = File::open("input.txt").expect("not found");
    let reader = BufReader::new(file);
    evaluate(parse(reader), true)
}

fn main() {
    println!("part_1: {}", part_1());
    println!("part_2: {}", part_2());
}
