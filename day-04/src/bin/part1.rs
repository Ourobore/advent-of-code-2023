use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Card(Vec<u32>);

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

fn parse_cards(inputs: Vec<String>) -> (Vec<Vec<u32>>, Vec<Card>) {
    let mut winning_numbers: Vec<Vec<u32>> = Vec::new();
    let mut cards: Vec<Card> = Vec::new();

    for line in inputs {
        let parsed_numbers = line.split(':').collect::<Vec<&str>>()[1]
            .split('|')
            .collect::<Vec<&str>>();
        // println!("{:?}", parsed_numbers);

        winning_numbers.push(
            parsed_numbers[0]
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<u32>().unwrap())
                .collect(),
        );
        cards.push(Card(
            parsed_numbers[1]
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<u32>().unwrap())
                .collect(),
        ));
    }

    (winning_numbers, cards)
}

fn compute_winnings(winning_numbers: Vec<Vec<u32>>, cards: Vec<Card>) -> u32 {
    let mut winnings: u32 = 0;
    for i in 0..winning_numbers.len() {
        // FIXME Could perhaps have done this in one go, with a fold() ?
        let found_winning_numbers = winning_numbers[i]
            .iter()
            .filter(|n| cards[i].0.contains(n))
            .collect::<Vec<&u32>>()
            .len() as u32;

        // FIXME Find a more elegant way
        if found_winning_numbers > 0 {
            let p: u32 = 2;
            winnings += p.pow(found_winning_numbers - 1);
        }
    }
    winnings
}

fn main() {
    let inputs = parse_input("input.txt");

    let (w, c) = parse_cards(inputs);
    // println!("{:?}", w);
    // println!("{:?}", c);

    let winnings = compute_winnings(w, c);
    println!("Winnings: {}", winnings);
}
