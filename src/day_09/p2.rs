/*
--- Part Two ---
Of course, it would be nice to have even more history included in your report.
Surely it's safe to just extrapolate backwards as well, right?

For each history, repeat the process of finding differences until the sequence
of differences is entirely zero. Then, rather than adding a zero to the end and
filling in the next values of each previous sequence, you should instead add a
zero to the beginning of your sequence of zeroes, then fill in new first values
for each previous sequence.

In particular, here is what the third example history looks like when
extrapolating back in time:

5  10  13  16  21  30  45
  5   3   3   5   9  15
   -2   0   2   4   6
      2   2   2   2
        0   0   0

Adding the new values on the left side of each sequence from bottom to top
eventually reveals the new left-most history value: 5.

Doing this for the remaining example data above results in previous values of -3
for the first history and 0 for the second history. Adding all three new values
together produces 2.

Analyze your OASIS report again, this time extrapolating the previous value for
each history. What is the sum of these extrapolated values?
*/

use regex::Regex;
use std::io::{BufRead, Write};

fn compute_differences(history: Vec<i32>) -> Vec<Vec<i32>> {
  let mut differences = vec![history];
  compute_differences_aux(&mut differences, 0);
  differences
}

fn compute_differences_aux(differences: &mut Vec<Vec<i32>>, diff_idx: usize) {
  if differences[diff_idx].iter().all(|diff| { *diff == 0 }) {
    return;
  }
  let mut next_diffs = Vec::new();
  for diff in differences[diff_idx].windows(2) {
    match diff {
      [v1, v2, ..] => next_diffs.push(v2 - v1),
      _ => panic!("Window should never be empty")
    }
  }
  differences.push(next_diffs);
  compute_differences_aux(differences, diff_idx + 1);
}

fn get_prediction(differences: &mut Vec<Vec<i32>>) -> i32 {
  // fill in the last step with a zero to start prediction.
  let mut value_below = 0;
  let last_step = differences
    .last_mut()
    .expect("Differences to not be empty");
  // FIXME: head insertion in vectors is inefficient
  last_step.insert(0, value_below);

  let mut prediction = 0;
  for step in differences.iter_mut().rev().skip(1) {
    prediction = step
      .first()
      .expect("The previous value in the step to exist") - value_below;
    step.push(prediction);
    value_below = prediction;
  }
  prediction
}

fn parse_input<R>(reader: R) -> Vec<Vec<i32>> where R: BufRead {
  let regex = Regex::new(r"[-+]?\d+").unwrap();
  let mut histories = Vec::new();
  for line in reader.lines() {
    let line = line.expect("Input line to exist");
    histories.push(
      regex.find_iter(&line).map(|n| {
        n.as_str().parse::<i32>().expect("Number to be parseable")
      }).collect()
    );
  }
  histories
}

pub fn solve<R, W>(reader: R, mut writer: W) where R: BufRead, W: Write {
  let histories = parse_input(reader);
  let solution: i32 = histories.into_iter().map(|h| {
    let mut diffs = compute_differences(h);
    get_prediction(&mut diffs)
  }).sum();

  write!(&mut writer, "The sum of all extrapolated values is: {}", solution).unwrap();
}

#[cfg(test)]
mod tests {
  
  use super::*;

  #[test]
  fn test_get_prediction() {
    let mut differences = vec![
      vec![10, 13, 16, 21, 30, 45],
      vec![3, 3, 5, 9, 15],
      vec![0, 2, 4, 6],
      vec![2, 2, 2],
      vec![0, 0],
    ];
    assert_eq!(get_prediction(&mut differences), 5);
  }
}