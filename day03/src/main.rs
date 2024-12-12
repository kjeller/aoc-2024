use parse_display::{Display, FromStr};
use regex::Regex;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;
#[derive(Display, FromStr, PartialEq, Debug)]
enum Instruction {
    #[display("mul({0},{1})")]
    Mul(i32, i32),
    #[display("do()")]
    Do(),
    #[display("don't()")]
    Dont(),
}
trait MemoryParser {
    fn parse_memory(&mut self, memory: &str);
}
#[derive(Default)]
struct SimpleCPU {
    instructions: Vec<Instruction>,
}
struct AdvancedCPU {
    instructions: Vec<Instruction>,
    enable: bool,
}
impl Default for AdvancedCPU {
   fn default() -> Self {
    AdvancedCPU {
        instructions : vec![],
        enable : true,
    }
   }
}
impl MemoryParser for SimpleCPU {
    fn parse_memory(&mut self, memory: &str) {
        if let Ok(re) = Regex::new(r"mul\(\d+,\d+\)") {
            for (full, []) in re.captures_iter(memory).map(|c| c.extract()) {
                if let Ok(instr) = Instruction::from_str(full) {
                    self.instructions.push(instr);
                }
            }
        }
    }
}
impl MemoryParser for AdvancedCPU {
    fn parse_memory(&mut self, memory: &str) {
        if let Ok(re) = Regex::new(r"mul\(\d+,\d+\)|do\(\)|don't\(\)") {
            for (full, []) in re.captures_iter(memory).map(|c| c.extract()) {
                if let Ok(instr) = Instruction::from_str(full) {
                    self.instructions.push(instr);
                }
            }
        }
    }
}
impl SimpleCPU {
    fn execute_instructions(&self) -> i32 {
        self.instructions
            .iter()
            .map(|instr| match instr {
                Instruction::Mul(x, y) => x * y,
                _ => 0,
            })
            .sum()
    }
}
impl AdvancedCPU {
    fn execute_instructions(&mut self) -> i32 {
        let mut res: Vec<i32> = vec![];
        self.instructions
            .iter()
            .for_each(|instr|{
                match instr {
                Instruction::Mul(x, y) => {
                    if self.enable {
                        res.push(x * y);
                    }
                }
                Instruction::Do() => self.enable = true,
                Instruction::Dont() => self.enable = false,
            }});
            res.iter().map(|c| c).sum()
    }
}
fn part_1() -> i32 {
    let file = File::open("input.txt").expect("not found");
    let reader = BufReader::new(file);
    let mut cpu: SimpleCPU = Default::default();
    reader.lines().for_each(|l| {
        if let Ok(l) = l {
            cpu.parse_memory(l.as_str());
        }
    });
    cpu.execute_instructions()
}
fn part_2() -> i32 {
    let file = File::open("input.txt").expect("not found");
    let reader = BufReader::new(file);
    let mut cpu: AdvancedCPU = Default::default();
    reader.lines().for_each(|l| {
        if let Ok(l) = l {
            cpu.parse_memory(l.as_str());
        }
    });
    cpu.execute_instructions()
}
fn main() {
    println!("part1: {}", part_1());
    println!("part2: {}", part_2());
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1_cpu() {
        let corrupt_memory =
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let mut cpu: SimpleCPU = Default::default();
        cpu.parse_memory(&corrupt_memory);
        assert_eq!(cpu.execute_instructions(), 161);
    }
    #[test]
    fn test_part2_cpu() {
        let corrupt_memory =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let mut cpu: AdvancedCPU = Default::default();
        cpu.parse_memory(&corrupt_memory);
        assert_eq!(cpu.execute_instructions(), 48);
    }
}
