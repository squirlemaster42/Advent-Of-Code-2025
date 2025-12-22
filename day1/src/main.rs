use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut zero_count = 0;
    let mut dial_pos = 50;

    println!("Dial At {}", dial_pos);
    for line in reader.lines() {
        let mut dir = 1;
        let line_str = line?;
        let mut iter = line_str.chars();
        if iter.nth(0).unwrap() == 'L' {
            dir = -1;
        }
        let str_value = &line_str[1..];
        let mut value = str_value.parse::<i32>().unwrap();
        while value > 0 {
            println!("Dial At {}", dial_pos);
            dial_pos += dir;

            if dial_pos % 100 == 0 {
                zero_count += 1;
            }
            value -= 1;
        }
    }

    println!("{}", zero_count);
    Ok(())
}
