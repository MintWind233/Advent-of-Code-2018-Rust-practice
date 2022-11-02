use std::fs::read_to_string;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = read_to_string("input/input.txt")?;
    part1(&input)?;
    Ok(())
}

// input: #123 @ 3,2: 5*4
// id:123, 3 inches from the left, 2 inches from the top
// 5*4 square
fn part1(input: &str) -> Result<()> {
    let mut storage_arr = Box::new([[0u16; 1000]; 1000]); 
    for line in input.lines() {
        let (from_left, from_top, width, height) = handle_line(line);
        for i in from_top..from_top+height {
            for j in from_left..from_left+width {
                storage_arr[i as usize][j as usize] = 1;
            } 
        }
    }
    println!("{:?}", storage_arr);
    Ok(())
}

pub fn handle_line(raw_line: &str) -> (i32, i32, i32, i32) {
    let temp: Vec<&str> = raw_line.split("@ ").collect();
    let temp = temp[1];
    let temp: Vec<&str> = temp.split(": ").collect();

    // 解析第一部分数据
    let first_part = temp[0];
    let first_temp: Vec<&str> = first_part.split(",").collect();
    let from_top: i32 = first_temp[0].parse().unwrap();
    let from_left: i32 = first_temp[1].parse().unwrap();

    // 解析第二部分数据
    let second_part = temp[1];
    let second_temp: Vec<&str> = second_part.split("x").collect();
    let width: i32 = second_temp[0].parse().unwrap();
    let height: i32 = second_temp[1].parse().unwrap();

    (from_top, from_left, width, height)
}
