use std::{fs, io};

fn main() -> io::Result<()> {
    let file_path = "../data.txt";
    let input_data = fs::read_to_string(file_path)?;

    let digits = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    let mut sum = 0;

    for line in input_data.lines() {
        // Parse corrected line as integer if possible
        let mut a = 0;
        for i in 0.. line.len() {
            let ch = line.chars().nth(i).unwrap();
            let mut done = false;
            if ch.is_digit(10) {
                a = (ch as i32 - '0' as i32) * 10;
                done = true;
            }
            if done { break;}
            for digit in digits.iter() {
                if line[i..].starts_with(digit.0) {
                    a = digit.1 * 10;
                    done = true;
                    break;
                }
            }
            if done { break;}
        }

        for i in (0..line.len()).rev() {
            let ch = line.chars().nth(i).unwrap();
            let mut done = false;
            if ch.is_digit(10) {
                a += ch as i32 - '0' as i32;
                done = true;
            }
            if done { break;}
            for digit in digits.iter() {
                if line[..i+1].ends_with(digit.0) {
                    a += digit.1;
                    done = true;
                    break;
                }
            }
            if done { break;}
        }
        println!("{}, {}", line,a);
        sum += a;
    }

    println!("Sum: {}", sum);
    Ok(())
}
