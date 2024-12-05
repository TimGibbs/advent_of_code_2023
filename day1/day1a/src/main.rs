use std::{fs, io};

fn main() -> io::Result<()> {
    let file_path = "../data.txt";
    let input_data = fs::read_to_string(file_path)?;

    let mut sum = 0;

    for line in input_data.lines() {
        let mut a = 0;
        for ch in line.chars() {
            if ch.is_digit(10) {
                a = (ch as i32 - '0' as i32) * 10;
                break;
            }
        }
        for ch in line.chars().rev() {
            if ch.is_digit(10) {
                a += ch as i32 - '0' as i32;
                break;
            }
        }
        sum += a;
    }
    println!("{}", sum);
    Ok(())
}
