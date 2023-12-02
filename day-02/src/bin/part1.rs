use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq)]
struct Round {
    red: u8,
    blue: u8,
    green: u8,
}

#[derive(Debug, PartialEq)]
struct Game {
    id: u8,
    rounds: Vec<Round>,
}

impl Game {
    fn from_string(str: &str) -> Game {
        let digit_pattern: &str = "\\d+";

        // TODO Add game string validator

        let game_id = match Regex::new(digit_pattern).unwrap().find(str) {
            Some(x) => x.as_str().parse::<u8>().unwrap(),
            None => panic!("No game id found"),
        };

        let mut rounds: Vec<Round> = Vec::new();

        let rounds_string = str.split(":").collect::<Vec<&str>>()[1];
        for round in rounds_string.split(";") {
            let red_count = match Regex::new([digit_pattern, " red"].concat().as_str())
                .unwrap()
                .find(round)
            {
                Some(x) => x.as_str().split(" ").collect::<Vec<&str>>()[0]
                    .parse::<u8>()
                    .unwrap(),
                None => 0,
            };
            let blue_count = match Regex::new([digit_pattern, " blue"].concat().as_str())
                .unwrap()
                .find(round)
            {
                Some(x) => x.as_str().split(" ").collect::<Vec<&str>>()[0]
                    .parse::<u8>()
                    .unwrap(),
                None => 0,
            };
            let green_count = match Regex::new([digit_pattern, " green"].concat().as_str())
                .unwrap()
                .find(round)
            {
                Some(x) => x.as_str().split(" ").collect::<Vec<&str>>()[0]
                    .parse::<u8>()
                    .unwrap(),
                None => 0,
            };

            rounds.push(Round {
                red: red_count,
                blue: blue_count,
                green: green_count,
            });
        }

        Game {
            id: game_id,
            rounds: rounds,
        }
    }

    fn is_game_possible(&self, red_count: u8, blue_count: u8, green_count: u8) -> bool {
        for round in &self.rounds {
            if round.red > red_count || round.blue > blue_count || round.green > green_count {
                return false;
            }
        }
        true
    }
}

fn parse_input(filename: &str) -> Vec<String> {
    let mut input: Vec<String> = Vec::new();

    let file = match File::open(filename) {
        Ok(x) => x,
        Err(e) => panic!("{}", e),
    };

    for line in BufReader::new(file).lines() {
        match line {
            Ok(x) => input.push(x),
            Err(e) => panic!("{}", e),
        };
    }
    input
}

fn sum_possible_games(inputs: Vec<String>) -> i32 {
    let mut games: Vec<Game> = Vec::new();
    for input in inputs {
        games.push(Game::from_string(input.as_str()));
    }
    // println!("{:?}", games);

    let possible_games = games
        .iter()
        .filter(|x| x.is_game_possible(12, 14, 13))
        .collect::<Vec<&Game>>();
    // println!("{:?}", possible_games);

    let possible_games_id_sum: i32 = possible_games
        .iter()
        .fold(0, |acc, x| acc + i32::from(x.id));
    possible_games_id_sum
}

fn main() {
    let inputs = parse_input("src/bin/input1.txt");

    let possible_games = sum_possible_games(inputs);
    println!("{}", possible_games)
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_game_from_string() {
        let game_string = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";

        assert_eq!(
            Game::from_string(game_string),
            Game {
                id: 1,
                rounds: vec![
                    Round {
                        blue: 3,
                        red: 4,
                        green: 0,
                    },
                    Round {
                        red: 1,
                        green: 2,
                        blue: 6,
                    },
                    Round {
                        green: 2,
                        red: 0,
                        blue: 0,
                    }
                ]
            }
        );
    }

    #[test]
    fn test_sum_possible_games() {
        let inputs: Vec<String> = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string(),
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string(),
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".to_string(),
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".to_string(),
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string(),
        ];

        assert_eq!(sum_possible_games(inputs), 8)
    }
}
