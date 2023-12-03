use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

// First is row, second is column
#[derive(Debug, Clone)]
struct Coordinates(usize, usize);

impl Coordinates {
    fn is_adjacent(&self, coordinates: Vec<Coordinates>) -> bool {
        coordinates
            .iter()
            .any(|x| self.0.abs_diff(x.0) <= 1 && self.1.abs_diff(x.1) <= 1)
    }
}

// Inner Vec is for storing a row, outer for storing multiple rows
#[derive(Debug, PartialEq)]
struct Schematic(Vec<Vec<char>>);

impl fmt::Display for Schematic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        for row in &self.0 {
            output.push_str(row.iter().collect::<String>().as_str());

            match &self.0.last() {
                Some(x) => {
                    if row != *x {
                        output.push('\n')
                    }
                }
                None => (),
            };
        }
        write!(f, "{}", output)
    }
}

impl Schematic {
    fn from_strings(strings: Vec<String>) -> Schematic {
        let mut rows: Vec<Vec<char>> = Vec::new();
        strings.iter().for_each(|x| rows.push(x.chars().collect()));

        Schematic(rows)
    }

    fn find_numbers(&self) -> Vec<(String, Vec<Coordinates>)> {
        // FIXME To refactor
        let mut numbers: Vec<(String, Vec<Coordinates>)> = Vec::new();
        for (row_number, row) in self.0.iter().enumerate() {
            let mut current_number: String = String::new();
            let mut current_number_coordinates: Vec<Coordinates> = Vec::new();
            for (char_number, c) in row.iter().enumerate() {
                if c.is_digit(10) {
                    current_number.push(c.to_owned());
                    current_number_coordinates.push(Coordinates(row_number, char_number));
                } else {
                    if current_number.len() > 0 {
                        numbers.push((current_number.clone(), current_number_coordinates.to_vec()));
                        current_number.clear();
                        current_number_coordinates.clear();
                    }
                }
            }
            if current_number.len() > 0 {
                numbers.push((current_number.clone(), current_number_coordinates.to_vec()));
                current_number.clear();
                current_number_coordinates.clear();
            }
        }
        numbers
    }

    fn find_symbols(&self) -> Vec<(String, Vec<Coordinates>)> {
        // FIXME To refactor
        let mut symbols: Vec<(String, Vec<Coordinates>)> = Vec::new();
        for (row_number, row) in self.0.iter().enumerate() {
            let mut current_symbol: String = String::new();
            let mut current_symbol_coordinates: Vec<Coordinates> = Vec::new();
            for (char_number, c) in row.iter().enumerate() {
                if c.is_ascii_punctuation() && *c != '.' && current_symbol.len() == 0 {
                    current_symbol.push(c.to_owned());
                    current_symbol_coordinates.push(Coordinates(row_number, char_number));
                } else {
                    if current_symbol.len() > 0 {
                        symbols.push((current_symbol.clone(), current_symbol_coordinates.to_vec()));
                        current_symbol.clear();
                        current_symbol_coordinates.clear();
                    }
                }
            }
            if current_symbol.len() > 0 {
                symbols.push((current_symbol.clone(), current_symbol_coordinates.to_vec()));
                current_symbol.clear();
                current_symbol_coordinates.clear();
            }
        }
        symbols
    }

    fn find_part_numbers(&self) -> Vec<u32> {
        let numbers = self.find_numbers();
        println!("{:?}", numbers.len());
        let symbols = self.find_symbols();
        println!("{:?}", symbols.len());
        let symbols_coordinates = symbols
            .iter()
            .flat_map(|s| s.1.to_vec())
            .collect::<Vec<Coordinates>>();

        // println!("{:?}", symbols_coordinates);

        let part_numbers: Vec<u32> = numbers
            .iter()
            .filter(|n| {
                n.1.iter()
                    .any(|coord| coord.is_adjacent(symbols_coordinates.to_vec()))
            })
            .map(|x| x.0.parse::<u32>().unwrap())
            .collect();

        part_numbers
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

fn sum_part_numbers(inputs: Vec<String>) -> u32 {
    let schematic = Schematic::from_strings(inputs);
    println!("{}", schematic);

    let part_numbers = schematic.find_part_numbers();
    println!("{:?}", part_numbers);

    let sum_part_numbers = part_numbers.iter().fold(0, |acc, x| acc + x);
    sum_part_numbers
}

fn main() {
    let inputs = parse_input("input1.txt");

    let sum = sum_part_numbers(inputs);
    println!("Sum part numbers: {}", sum);
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_from_strings() {
        let inputs: Vec<String> = vec![
            "12.ab".to_string(),
            "..*78".to_string(),
            "*294.".to_string(),
        ];

        assert_eq!(
            Schematic::from_strings(inputs),
            Schematic(vec![
                vec!['1', '2', '.', 'a', 'b'],
                vec!['.', '.', '*', '7', '8'],
                vec!['*', '2', '9', '4', '.'],
            ])
        )
    }

    #[test]
    fn test_big_from_strings() {
        let inputs = parse_input("input1.txt");
        let schematic = Schematic::from_strings(inputs.to_vec());
        let mut string_vec: Vec<String> = Vec::new();
        for row in schematic.0 {
            string_vec.push(row.iter().collect());
        }

        assert_eq!(string_vec, inputs);
    }

    #[test]
    fn test_coordonates_is_adjacent() {
        let c = Coordinates(1, 1);

        assert_eq!(c.is_adjacent(vec![Coordinates(1, 2)]), true);
        assert_eq!(c.is_adjacent(vec![Coordinates(1, 3)]), false);

        assert_eq!(
            c.is_adjacent(vec![Coordinates(1, 2), Coordinates(1, 3)]),
            true
        );
    }

    #[test]
    fn test_sum_part_numbers_is_correct() {
        let inputs = parse_input("testinput.txt");

        assert_eq!(sum_part_numbers(inputs), 4361)
    }
}
