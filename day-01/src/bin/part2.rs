use std::fs::File;
use std::io::{BufRead, BufReader};

const NUMBERS: [&'static str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

const DIGITS: [&'static str; 9] = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];

fn decode_digit(str: &str) -> u8 {
    match str {
        "one" | "1" => 1,
        "two" | "2" => 2,
        "three" | "3" => 3,
        "four" | "4" => 4,
        "five" | "5" => 5,
        "six" | "6" => 6,
        "seven" | "7" => 7,
        "eight" | "8" => 8,
        "nine" | "9" => 9,
        &_ => panic!("Invalid input"),
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

fn parse_two_digit_number(str: String) -> i32 {
    let mut number_pos: Vec<(usize, &str)> = Vec::new();
    for number in [NUMBERS, DIGITS].concat() {
        let matches: Vec<_> = str.match_indices(number).collect();
        number_pos = [number_pos, matches].concat();
    }
    number_pos.sort_by_key(|k| k.0);
    // println!("{:?}", number_pos);

    let first_digit = match number_pos.first() {
        Some(x) => decode_digit(x.1),
        None => 0,
    };
    let second_digit = match number_pos.last() {
        Some(x) => decode_digit(x.1),
        None => 0,
    };

    format!("{}{}", first_digit, second_digit)
        .parse::<i32>()
        .unwrap()
}

fn sum_inputs(inputs: Vec<String>) -> i32 {
    let mut numbers: Vec<i32> = Vec::new();
    for str in inputs {
        numbers.push(parse_two_digit_number(str));
    }
    // println!("{:?}", numbers);

    let sum: i32 = numbers.iter().sum();
    sum
}

fn main() {
    let input = parse_input("src/bin/input2.txt");
    // println!("{:?}", input);

    let sum = sum_inputs(input);
    println!("{}", sum)
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_sum_inputs_with_test_input() {
        let inputs: Vec<String> = vec![
            String::from("two1nine"),
            String::from("eightwothree"),
            String::from("abcone2threexyz"),
            String::from("xtwone3four"),
            String::from("4nineeightseven2"),
            String::from("zoneight234"),
            String::from("7pqrstsixteen"),
        ];

        assert_eq!(sum_inputs(inputs), 281);
    }

    #[test]
    fn test_sum_inputs_with_input_without_digits() {
        let inputs: Vec<String> = vec![String::from("abc")];

        assert_eq!(sum_inputs(inputs), 0);
    }

    #[test]
    fn test_sum_inputs_with_input_with_digits_0() {
        let inputs: Vec<String> = vec![String::from("0abc0")];

        assert_eq!(sum_inputs(inputs), 0);
    }
}
