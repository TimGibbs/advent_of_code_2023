use std::{fs, io};
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let file_path = "../data.txt";
    let input_data = fs::read_to_string(file_path)?;

    // Parse the data
    let mut games: HashMap<u32, Vec<HashMap<String, u32>>> = HashMap::new();

    for line in input_data.lines() {
        if let Some((game, data)) = line.split_once(": ") {
            // Extract game number
            let game_number: u32 = game.replace("Game ", "").parse().unwrap_or(0);

            // Parse the rounds
            let rounds = data.split("; ").map(|round| {
                round.split(", ").fold(HashMap::new(), |mut acc, item| {
                    if let Some((count, color)) = item.split_once(' ') {
                        let count: u32 = count.parse().unwrap_or(0);
                        *acc.entry(color.to_string()).or_insert(0) += count;
                    }
                    acc
                })
            }).collect();

            games.insert(game_number, rounds);
        }
    }

    let mut sum = 0;
    
    let red_limit = 12;
    let green_limit = 13;
    let blue_limit = 14;
    for (game_number, rounds) in &games {
        let mut valid = true;
        for (_, round) in rounds.iter().enumerate() {
            if round.get("red").is_some() && round["red"] > red_limit { valid = false; break; }
            if round.get("green").is_some() && round["green"] > green_limit { valid = false; break; }
            if round.get("blue").is_some() && round["blue"] > blue_limit { valid = false; break; }
            if !valid { break; }
        }
        if valid { sum+= game_number; }
    }
    
    println!("{}", sum);
    Ok(())
}