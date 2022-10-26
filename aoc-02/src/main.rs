use std::{
    collections::{HashMap, HashSet},
    fs,
};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
use std::io::{self,Write};

fn main() -> Result<()> {
    let input = fs::read_to_string("input/input.txt")?;

    check_code(&input)?;
    the_most_similar_id(&input)?;

    Ok(())
}

// part1代码
// 思路就是HashMap记录出现次数
pub fn check_code(input: &str) -> Result<()> {
    let (mut two, mut three) = (0, 0);
    for line in input.lines() {
        let mut mp = HashMap::new();

        for chr in line.chars() {
            let count = mp.entry(chr).or_insert(0);
            *count += 1;
        }

        let set_val: HashSet<i32> = mp.into_values().collect();

        if set_val.contains(&2) {
            two += 1;
        };

        if set_val.contains(&3) {
            three += 1;
        };
    }
    println!("{}", two * three);
    Ok(())
}

// part2 找等长字符串的最长公共字符串
pub fn the_most_similar_id(input: &str) -> Result<()> {
    let vec: Vec<&str> = input.lines().collect();
    for i in 0..vec.len() {
        for j in i + 1..vec.len() {
            if let Some(common) = similar_charat(vec[i], vec[j]) {
                writeln!(io::stdout(), "{}", common)?;
                return Ok(());
            }
        }
    }
    Ok(())
}

// 寻找两个等长字符串之间有几个字符相同
pub fn similar_charat(str1: &str, str2: &str) -> Option<String> {
    let mut checher = false;
    // 连续有两个字符不同就不找了
    for (c1, c2) in str1.chars().zip(str2.chars()) {
        if c1 != c2 {
            if checher {
                return None;
            }
            checher = true;
        }
    }
    // 把相同的字符收集起来，最后collect
    Some(
        str1.chars()
            .zip(str2.chars())
            .filter(|&(c1, c2)| c1 == c2)
            .map(|(c, _)| c)
            .collect(),
    )
}
