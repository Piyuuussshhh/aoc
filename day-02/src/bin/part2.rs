use std::{collections::HashMap, u32};

fn main() {
    let test_cases = include_str!("./input1.txt");
    let output = part2(test_cases.trim());
    dbg!(output);
}

fn part2(games: &str) -> u32 {
    // let mut config: HashMap<&str, i32> = HashMap::new();
    // config.insert("red", 12);
    // config.insert("green", 13);
    // config.insert("blue", 14);

    let mut sum: u32 = 0;
    for game in games.lines() {
        // Extracting id
        let mut spaces = 0;
        let mut idx = 0;
        for letter in game.chars() {
            if letter == ' ' {
                spaces += 1;
                if spaces > 1 {
                    break;
                }
            }
            idx += 1;
        }
        // Extracting the colors and numbers of balls in this game.
        let sets: Vec<&str> = game[(idx + 1)..].split(';').collect();
        dbg!(sets.clone());
        let mut max_red: i32 = 0;
        let mut max_green: i32 = 0;
        let mut max_blue: i32 = 0;

        for set in sets.iter() {
            let mut game_config: HashMap<&str, i32> = HashMap::new();
            game_config.insert("red", 0);
            game_config.insert("green", 0);
            game_config.insert("blue", 0);
            let balls: Vec<&str> = set.split(',').collect();
            println!("{:#?}", balls);
            for ball in balls {
                let temp = ball.trim();

                // Split temp into ["number", "color"];
                let info: Vec<&str> = temp.split(' ').collect();

                let freq = game_config.entry(&info[1]).or_insert(0);
                *freq += info[0].parse::<i32>().unwrap();
            }

            dbg!(game_config.clone());
            // Need to compare the game_config["color"] with max_color, and assign if greater.
            max_red = if game_config["red"] > max_red {
                game_config["red"]
            } else {
                max_red
            };
            max_green = if game_config["green"] > max_green {
                game_config["green"]
            } else {
                max_green
            };
            max_blue = if game_config["blue"] > max_blue {
                game_config["blue"]
            } else {
                max_blue
            };
        }
        println!("Max color freqs for this game: Red = {max_red}\tGreen = {max_green}\tBlue = {max_blue}");
        // Here we get the max_colors, we multiply them and add it to sum
        sum += max_red as u32 * max_blue as u32 * max_green as u32;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(result, 2286u32)
    }
}
