use std::{io::{self, Write}, collections::HashSet};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/input.txt")?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

// 第一个问题，逻辑比较简单，从0开始相加，处理下txt的IO就可以了，
fn part1(input: &str) -> Result<()> {
    let mut freq = 0;
    for line in input.lines() {
        let change: i32 = line.parse()?;
        freq += change;
    }
    writeln!(io::stdout(), "{}", freq)?;
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut freq = 0;
    let mut seen = HashSet::new();
    seen.insert(0);

    loop {
        for line in input.lines() {
            let change: i32 = line.parse()?;
            freq += change;
            if seen.contains(&freq) {
                writeln!(io::stdout(), "{}", freq)?;
                return Ok(());
            }
            seen.insert(freq);
        }
    }
}
