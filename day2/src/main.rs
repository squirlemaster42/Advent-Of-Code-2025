use std::fs::{self};
use std::error::Error;
use std::str;

fn main() -> Result<(), Box<dyn Error>> {
    let message: String = fs::read_to_string("sample.txt")?;
    println!("{}", message);
    let parts = message.split(",");
    let mut sum = 0;
    for part in parts {
        let split_range: Vec<&str> = part.split("-").collect();
        println!("{:?}", split_range);
        let min: u64 = split_range[0].trim().parse().expect("not a number");
        let max: u64 = split_range[1].trim().parse().expect("not a number");

        for i in min..=max {
            let mut found = 0;
            let mut num = i;
            while num != 0 {
                let digit = num % 10;
                num = num / 10;
                println!("Digit: {} - Found: {}", digit, found);
                if (found & (1 << digit)) > 0 {
                    println!("Found num with dup digits: {}", i);
                    sum += i;
                    break
                }
                found = found | (1 << digit)
            }
        }
    }

    println!("{}", sum);
    Ok(())
}
