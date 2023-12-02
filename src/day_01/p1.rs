use std::io::{BufRead, Write};

/**
--- Day 1: Trebuchet?! ---
Something is wrong with global snow production, and you've been selected to take a look. The Elves have even given you a map; on it, they've used stars to mark the top fifty locations that are likely to be having problems.

You've been doing this long enough to know that to restore snow operations, you need to check all fifty stars by December 25th.

Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!

You try to ask why they can't just use a weather machine ("not powerful enough") and where they're even sending you ("the sky") and why your map looks mostly blank ("you sure ask a lot of questions") and hang on did you just say the sky ("of course, where do you think snow comes from") when you realize that the Elves are already loading you into a trebuchet ("please hold still, we need to strap you in").

As they're making the final adjustments, they discover that their calibration document (your puzzle input) has been amended by a very young Elf who was apparently just excited to show off her art skills. Consequently, the Elves are having trouble reading the values on the document.

The newly-improved calibration document consists of lines of text; each line originally contained a specific calibration value that the Elves now need to recover. On each line, the calibration value can be found by combining the first digit and the last digit (in that order) to form a single two-digit number.

For example:

1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
In this example, the calibration values of these four lines are 12, 38, 15, and 77. Adding these together produces 142.

Consider your entire calibration document. What is the sum of all of the calibration values?
*/

pub fn parse_input<R>(reader: R) -> Vec<u32> where R: BufRead {
    reader.lines().map(|l| {
        // NB: assumes unicode
        let l = l.expect("Line to be valid");
        let mut iter = l.chars();
        let mut rev_iter = l.chars().rev();
        let first = iter
            .find(|c| c.is_ascii_digit())
            .expect("A digit to exist")
            .to_digit(10)
            .expect("To be representable in base 10");
        let last = rev_iter
            .find(|c| c.is_ascii_digit())
            .expect("A digit to exist")
            .to_digit(10)
            .expect("To be representable in base 10");
        first * 10 + last
    }).collect()
}

pub fn solve<R, W>(reader: R, mut writer: W) where R: BufRead, W: Write {
    let solution: u32 = parse_input(reader).iter().sum();

    write!(&mut writer, "The sum of all calibration values is: {}", solution).unwrap();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = b"1abc2\n\
                                 pqr3stu8vwx";
        assert_eq!(parse_input(&input[..]), vec![12, 38]);
    }

    #[test]
    fn test_solve() {
        let input = b"1abc2\n\
                     pqr3stu8vwx\n\
                     a1b2c3d4e5f\n\
                     treb7uchet";
        let mut output = Vec::new();

        solve(&input[..], &mut output);

        assert_eq!(String::from_utf8(output).unwrap(), "The sum of all calibration values is: 142");
    }
}