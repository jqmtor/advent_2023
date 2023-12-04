use regex::Regex;
use std::collections::HashSet;
use std::io::{BufRead, Write};

/*
--- Day 4: Scratchcards ---
The gondola takes you up. Strangely, though, the ground doesn't seem to be
coming with you; you're not climbing a mountain. As the circle of Snow Island
recedes below you, an entire new landmass suddenly appears above you! The
gondola carries you to the surface of the new island and lurches into the
station.

As you exit the gondola, the first thing you notice is that the air here is much
warmer than it was on Snow Island. It's also quite humid. Is this where the
water source is?

The next thing you notice is an Elf sitting on the floor across the station in
what seems to be a pile of colorful square cards.

"Oh! Hello!" The Elf excitedly runs over to you. "How may I be of service?" You
ask about water sources.

"I'm not sure; I just operate the gondola lift. That does sound like something
we'd have, though - this is Island Island, after all! I bet the gardener would
know. He's on a different island, though - er, the small kind surrounded by
water, not the floating kind. We really need to come up with a better naming
scheme. Tell you what: if you can help me with something quick, I'll let you
borrow my boat and you can go visit the gardener. I got all these scratchcards
as a gift, but I can't figure out what I've won."

The Elf leads you over to the pile of colorful cards. There, you discover dozens
of scratchcards, all with their opaque covering already scratched off. Picking
one up, it looks like each card has two lists of numbers separated by a vertical
bar (|): a list of winning numbers and then a list of numbers you have. You
organize the information into a table (your puzzle input).

As far as the Elf has been able to figure out, you have to figure out which of
the numbers you have appear in the list of winning numbers. The first match
makes the card worth one point and each match after the first doubles the point
value of that card.

For example:

Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11

In the above example, card 1 has five winning numbers (41, 48, 83, 86, and 17)
and eight numbers you have (83, 86, 6, 31, 17, 9, 48, and 53). Of the numbers
you have, four of them (48, 83, 17, and 86) are winning numbers! That means card
1 is worth 8 points (1 for the first match, then doubled three times for each of
the three matches after the first).

Card 2 has two winning numbers (32 and 61), so it is worth 2 points.  Card 3 has
two winning numbers (1 and 21), so it is worth 2 points.  Card 4 has one winning
number (84), so it is worth 1 point.  Card 5 has no winning numbers, so it is
worth no points.  Card 6 has no winning numbers, so it is worth no points.  So,
in this example, the Elf's pile of scratchcards is worth 13 points.

Take a seat in the large pile of colorful cards. How many points are they worth
in total?
*/

#[derive(Debug, PartialEq)]
pub struct ScratchCard {
    numbers: Vec<u32>,
    winning_numbers: HashSet<u32>,
}

fn count_winning(card: &ScratchCard) -> usize {
    card.numbers.iter().filter(|n| card.winning_numbers.contains(n)).count()
}

fn parse_card(card: &str) -> ScratchCard {
    let mut header_split = card.split(':');
    // discard card header
    header_split.next().expect("Card header to be present");
    let card_contents = header_split.next().expect("Card contents to be present");
    let mut contents_split = card_contents.split('|');
    let winning_numbers = contents_split.next().expect("Winning numbers to be present");
    let numbers = contents_split.next().expect("Card numbers to be present");

    let regex = Regex::new(r"\d+",).unwrap();
    let winning_numbers: Vec<u32> = regex
        .find_iter(winning_numbers)
        .map(|n| {
            n.as_str().parse::<u32>().expect("Number to be parseable")
        }).collect();

    let numbers = regex
        .find_iter(numbers)
        .map(|n| {
            n.as_str().parse::<u32>().expect("Number to be parseable")
        }).collect();

    ScratchCard { numbers, winning_numbers: HashSet::from_iter(winning_numbers) }
}

fn parse_input<R>(reader: R) -> Vec<ScratchCard> where R: BufRead {
    reader.lines().map(|l| {
        let l = l.expect("Line to be present");
        parse_card(&l)
    }).collect()
}

pub fn solve<R, W>(reader: R, mut writer: W) where R: BufRead, W: Write {
    let solution: u32 = parse_input(reader)
        .iter()
        .map(|c| {
            u32::try_from(count_winning(c))
                .expect("Number to be conversible.")
        })
        .map(|count| {
            // since the count starts at one, and always doubles after that,
            // it's the same as computing 2^count-1, with count = 0 being the
            // exception
            if count == 0 {
                count
            } else {
                2_u32.pow(count - 1)
            }
        }).sum();

    write!(&mut writer, "The pile of the Elf's scratchcards is worth: {}", solution).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_card() { 
        let line = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let expected = ScratchCard {
            winning_numbers: [41, 48, 83, 86, 17].into(),
            numbers: vec![83, 86, 6, 31, 17, 9, 48, 53]
        };

        assert_eq!(parse_card(line), expected);
    }

    #[test]
    fn test_count_winning_without_matches() {
        let card = ScratchCard {
            numbers: vec![1, 3, 5, 7, 9],
            winning_numbers: [2, 4, 6, 8].into()
        };
        assert_eq!(count_winning(&card), 0);
    }

    #[test]
    fn test_count_winning_with_matches() {
        let card = ScratchCard {
            numbers: vec![1, 2, 5, 8, 9],
            winning_numbers: [2, 4, 6, 8].into()
        };
        assert_eq!(count_winning(&card), 2);
    }
}