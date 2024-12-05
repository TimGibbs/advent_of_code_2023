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

    for (_, rounds) in &games {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for (_, round) in rounds.iter().enumerate() {
            if round.get("red").is_some() && round["red"] > red { red = round["red"] }
            if round.get("green").is_some() && round["green"] > green { green = round["green"] }
            if round.get("blue").is_some() && round["blue"] > blue { blue = round["blue"] }
        }
        sum += red * green * blue;
    }

    println!("{}", sum);
    Ok(())
}