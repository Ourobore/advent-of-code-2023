use std::fs::File;
use std::io::{BufRead, BufReader};

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
    let first_digit = match str.chars().filter(|c| c.is_digit(10)).next() {
        Some(x) => x,
        None => '0',
    };
    let second_digit = match str.chars().rev().filter(|c| c.is_digit(10)).next() {
        Some(x) => x,
        None => '0',
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
    let input = parse_input("src/bin/input1.txt");
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
            String::from("1abc2"),
            String::from("pqr3stu8vwx"),
            String::from("a1b2c3d4e5f"),
            String::from("treb7uchet"),
        ];

        assert_eq!(sum_inputs(inputs), 142);
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
