use std::io::{BufRead, Write};

/*
--- Day 2: Cube Conundrum ---
You're launched high into the atmosphere! The apex of your trajectory just
barely reaches the surface of a large island floating in the sky. You gently
land in a fluffy pile of leaves. It's quite cold, but you don't see much snow.
An Elf runs over to greet you.

The Elf explains that you've arrived at Snow Island and apologizes for the lack
of snow. He'll be happy to explain the situation, but it's a bit of a walk, so
you have some time. They don't get many visitors up here; would you like to play
a game in the meantime?

As you walk, the Elf shows you a small bag and some cubes which are either red,
green, or blue. Each time you play this game, he will hide a secret number of
cubes of each color in the bag, and your goal is to figure out information about
the number of cubes.

To get information, once a bag has been loaded with cubes, the Elf will reach
into the bag, grab a handful of random cubes, show them to you, and then put
them back in the bag. He'll do this a few times per game.

You play several games and record the information from each game (your puzzle
input). Each game is listed with its ID number (like the 11 in Game 11: ...)
followed by a semicolon-separated list of subsets of cubes that were revealed
from the bag (like 3 red, 5 green, 4 blue).

For example, the record of a few games might look like this:

Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green Game 2: 1 blue, 2 green;
3 green, 4 blue, 1 red; 1 green, 1 blue Game 3: 8 green, 6 blue, 20 red; 5 blue,
4 red, 13 green; 5 green, 1 red Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red;
3 green, 15 blue, 14 red Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
In game 1, three sets of cubes are revealed from the bag (and then put back
again). The first set is 3 blue cubes and 4 red cubes; the second set is 1 red
cube, 2 green cubes, and 6 blue cubes; the third set is only 2 green cubes.

The Elf would first like to know which games would have been possible if the bag
contained only 12 red cubes, 13 green cubes, and 14 blue cubes?

In the example above, games 1, 2, and 5 would have been possible if the bag had
been loaded with that configuration. However, game 3 would have been impossible
because at one point the Elf showed you 20 red cubes at once; similarly, game 4
would also have been impossible because the Elf showed you 15 blue cubes at
once. If you add up the IDs of the games that would have been possible, you get
8.

Determine which games would have been possible if the bag had been loaded with
only 12 red cubes, 13 green cubes, and 14 blue cubes. What is the sum of the IDs
of those games?
*/


#[derive(Debug, Default, PartialEq)]
pub struct CubeSample {
    pub blue: u32,
    pub green: u32,
    pub red: u32,
}

#[derive(Debug)]
pub struct Game {
    pub id: u32,
    pub samples: Vec<CubeSample>,
}

fn parse_sample(sample: &str) -> CubeSample {
    let mut cube_sample: CubeSample = Default::default();
    for cube in sample.split(',') {
        let cube = cube.trim();
        let mut cube_split = cube.split(' ');
        let quantity: u32 = cube_split
            .next()
            .expect("The sample quantity to be present")
            .trim()
            .parse()
            .expect("Sample quantity to be parseable");
        let color = cube_split.next().expect("The cube color to be present");
        match color {
            "blue" => cube_sample.blue = quantity,
            "green" => cube_sample.green = quantity,
            "red" => cube_sample.red = quantity,
            _ => panic!("Unsupported color")
        }
    }
    cube_sample
}

fn parse_samples(samples: &str) -> Vec<CubeSample> {
    samples
        .split(';')
        .map(parse_sample)
        .collect()
}

fn parse_game(game: String) -> Game {
    let mut split = game.split(':');
    let game_header = split.next().expect("Game header to be present.");
    let samples = split.next().expect("Cube samples to be present").trim();

    let id: u32 = game_header
        .split(' ')
        .nth(1)
        .expect("Game ID to be present")
        .parse()
        .expect("Game ID to be parseable");
    let samples = parse_samples(samples);

    Game {id, samples}
}

pub fn parse_input<R>(reader: R) -> Vec<Game> where R: BufRead {
    reader.lines().map(|l| {
        let l = l.expect("Line to be present");
        parse_game(l)
    }).collect()
}

pub fn solve<R, W>(reader: R, mut writer: W) where R: BufRead, W: Write {
    let reference_sample = CubeSample {
        green: 13,
        blue: 14,
        red: 12,
    };

    let solution: u32 = parse_input(reader)
        .iter()
        .filter(|g| {
            g.samples.iter().all(|s| {
                s.blue <= reference_sample.blue &&
                s.green <= reference_sample.green &&
                s.red <= reference_sample.red
            })
        }).map(|g| g.id)
        .sum();

    write!(&mut writer, "The sum of all possible game IDs is: {}", solution).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = b"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";

        let expected_sample = CubeSample { blue: 3, green: 0, red: 4};
        let expected_game_id = 1;

        let actual_games = parse_input(&input[..]);
        let actual_game = actual_games.first().unwrap();
        let actual_sample = actual_game.samples.first().unwrap();

        assert_eq!(actual_game.id, expected_game_id);
        assert_eq!(*actual_sample, expected_sample);
    }

    #[test]
    fn test_solve() {
            let input = b"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
                                      Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
                                      Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
                                      Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
                                      Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
            let mut output = Vec::new();

            solve(&input[..], &mut output);

        assert_eq!(String::from_utf8(output).unwrap(), "The sum of all possible game IDs is: 8");
    }
}
