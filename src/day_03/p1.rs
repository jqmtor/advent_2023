use regex::Regex;
// use std::io::{BufRead, Write};
use std::collections::HashSet;

/*
--- Day 3: Gear Ratios ---
You and the Elf eventually reach a gondola lift station; he says the gondola
lift will take you up to the water source, but this is as far as he can bring
you. You go inside.

It doesn't take long to find the gondolas, but there seems to be a problem:
they're not moving.

"Aaah!"

You turn around to see a slightly-greasy Elf with a wrench and a look of
surprise. "Sorry, I wasn't expecting anyone! The gondola lift isn't working
right now; it'll still be a while before I can fix it." You offer to help.

The engineer explains that an engine part seems to be missing from the engine,
but nobody can figure out which one. If you can add up all the part numbers in
the engine schematic, it should be easy to work out which part is missing.

The engine schematic (your puzzle input) consists of a visual representation of
the engine. There are lots of numbers and symbols you don't really understand,
but apparently any number adjacent to a symbol, even diagonally, is a "part
number" and should be included in your sum. (Periods (.) do not count as a
symbol.)

Here is an example engine schematic:

467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..

In this schematic, two numbers are not part numbers because they are not
adjacent to a symbol: 114 (top right) and 58 (middle right). Every other number
is adjacent to a symbol and so is a part number; their sum is 4361.

Of course, the actual engine schematic is much larger. What is the sum of all of
the part numbers in the engine schematic?
*/

#[derive(Debug, PartialEq)]
pub struct SchematicLine {
    pub candidates: Vec<SchematicNumber>,
    pub symbol_locations: HashSet<usize>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SchematicNumber {
    pub number: u32,
    pub adjacency_range: HashSet<usize>,
}

fn parse_line(line: &str) -> SchematicLine {
    let number_regex = Regex::new(r"\d+",).unwrap();
    let numbers: Vec<SchematicNumber> = number_regex.find_iter(line).map(|n| {
        let mut range = n.range();
        if range.start > 0 {
            range.start -= 1;
        };

        if range.end < line.len() - 1 {
            range.end += 1;
        };

        SchematicNumber {
            number: n.as_str().parse::<u32>().unwrap(),
            adjacency_range: HashSet::from_iter(range),
        }
    }).collect();

    let symbol_regex = Regex::new(r"[*#+$]").unwrap();

    let symbol_locations = symbol_regex.find_iter(line).map(|n| {
        n.range().start
    }).collect();

    SchematicLine {
        candidates: numbers,
        symbol_locations,
    }
}

fn is_symbol_adjacent(number: &SchematicNumber, line_idx: usize, lines: &Vec<SchematicLine>) -> bool {
    !lines[line_idx].symbol_locations.is_disjoint(&number.adjacency_range)
    || line_idx > 0 && is_symbol_adjacent(number, line_idx - 1, lines)
    || line_idx < lines.len() - 1 && is_symbol_adjacent(number, line_idx + 1, lines)
}

pub fn find_parts(lines: Vec<SchematicLine>) -> Vec<SchematicNumber> {
    let mut parts: Vec<SchematicNumber> = Vec::new();
    for (line_idx, line) in lines.iter().enumerate() {
        for candidate in line.candidates.iter() {
            if is_symbol_adjacent(candidate, line_idx, &lines) {
                parts.push(candidate.clone());
            }
        }
    }
    parts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let line = "467..114..";
        let expected = SchematicLine {
            candidates: vec![
                SchematicNumber {
                    number: 467,
                    adjacency_range: HashSet::from([0, 1, 2, 3]),
                },
                SchematicNumber {
                    number: 114,
                    adjacency_range: HashSet::from([4, 5, 6, 7, 8]),
                }
            ],
            symbol_locations: HashSet::from([]),
        };
        let actual = parse_line(line);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_find_parts() {
        // Visual schematic for the test
        // 10*...
        let number = SchematicNumber {
            number: 10,
            adjacency_range: HashSet::from([0, 1, 2]),
        };
        let line = SchematicLine {
            candidates: vec![number.clone()],
            symbol_locations: HashSet::from([2]),
        };
        let actual_parts = find_parts(vec![line]);
        let actual_number  = actual_parts.first().unwrap();
        assert_eq!(number, *actual_number);
    }
}
