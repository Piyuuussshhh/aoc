use std::{collections::HashMap, u32};

fn main() {
    let test_cases = include_str!("./input1.txt");
    let output = part1(test_cases.trim());
    dbg!(output);
}

// I hate this code i hate it. I absolutely hate it but it works. I need to refactor THE FUCK out
// of this istg i suck at rust.

fn part1(games: &str) -> u32 {
    let mut config: HashMap<&str, i32> = HashMap::new();
    config.insert("red", 12);
    config.insert("green", 13);
    config.insert("blue", 14);

    let mut sum: u32 = 0;
    for game in games.lines() {
        // Extracting id
        let mut game_id: u32 = 0;
        let mut spaces = 0;
        let mut idx = 0;
        for letter in game.chars() {
            if letter == ' ' {
                spaces += 1;
                if spaces > 1 {
                    break;
                }
            }
            if letter.is_numeric() {
                game_id = game_id * 10 + letter.to_digit(10).unwrap();
            }
            idx += 1;
        }
        println!("Game id: {game_id}");
        // Extracting the colors and numbers of balls in this game.
        let sets: Vec<&str> = game[(idx + 1)..].split(';').collect();
        dbg!(sets.clone());
        let mut valid_game: bool = true;

        for set in sets.iter() {
            let mut game_config: HashMap<&str, i32> = HashMap::new();
            game_config.insert("red", 0);
            game_config.insert("green", 0);
            game_config.insert("blue", 0);
            let balls: Vec<&str> = set.split(',').collect();
            println!("{:#?}", balls);
            for ball in balls {
                let temp = ball.trim();
                //let num = temp.chars().nth(0).unwrap().to_digit(10).unwrap();
                //let color = &temp[2..];

                // Split temp into ["number", "color"];
                let info: Vec<&str> = temp.split(' ').collect();

                let freq = game_config.entry(&info[1]).or_insert(0);
                *freq += info[0].parse::<i32>().unwrap();
            }

            dbg!(game_config.clone());
            if !(game_config["green"] <= config["green"]
                && game_config["red"] <= config["red"]
                && game_config["blue"] <= config["blue"])
            {
                valid_game = false;
            }
            dbg!(valid_game);
        }

        if valid_game {
            println!("adding sum with {game_id}");
            sum += game_id;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(result, 8u32)
    }
}
