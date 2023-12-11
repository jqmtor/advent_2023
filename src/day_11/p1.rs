/*
--- Day 11: Cosmic Expansion ---
You continue following signs for "Hot Springs" and eventually come across an
observatory. The Elf within turns out to be a researcher studying cosmic
expansion using the giant telescope here.

He doesn't know anything about the missing machine parts; he's only visiting for
this research project. However, he confirms that the hot springs are the
next-closest area likely to have people; he'll even take you straight there once
he's done with today's observation analysis.

Maybe you can help him with the analysis to speed things up?

The researcher has collected a bunch of data and compiled the data into a single
giant image (your puzzle input). The image includes empty space (.) and galaxies
(#). For example:

...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....

The researcher is trying to figure out the sum of the lengths of the shortest
path between every pair of galaxies. However, there's a catch: the universe
expanded in the time it took the light from those galaxies to reach the
observatory.

Due to something involving gravitational effects, only some space expands. In
fact, the result is that any rows or columns that contain no galaxies should all
actually be twice as big.

In the above example, three columns and two rows contain no galaxies:

   v  v  v
 ...#......
 .......#..
 #.........
>..........<
 ......#...
 .#........
 .........#
>..........<
 .......#..
 #...#.....
   ^  ^  ^

These rows and columns need to be twice as big; the result of cosmic expansion
therefore looks like this:

....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#.......

Equipped with this expanded universe, the shortest path between every pair of
galaxies can be found. It can help to assign every galaxy a unique number:

....1........
.........2...
3............
.............
.............
........4....
.5...........
............6
.............
.............
.........7...
8....9.......

In these 9 galaxies, there are 36 pairs. Only count each pair once; order within
the pair doesn't matter. For each pair, find any shortest path between the two
galaxies using only steps that move up, down, left, or right exactly one . or #
at a time. (The shortest path between two galaxies is allowed to pass through
another galaxy.)

For example, here is one of the shortest paths between galaxies 5 and 9:

....1........
.........2...
3............
.............
.............
........4....
.5...........
.##.........6
..##.........
...##........
....##...7...
8....9.......

This path has length 9 because it takes a minimum of nine steps to get from
galaxy 5 to galaxy 9 (the eight locations marked # plus the step onto galaxy 9
itself). Here are some other example shortest path lengths:

Between galaxy 1 and galaxy 7: 15
Between galaxy 3 and galaxy 6: 17
Between galaxy 8 and galaxy 9: 5

In this example, after expanding the universe, the sum of the shortest path
between all 36 pairs of galaxies is 374.

Expand the universe, then find the length of the shortest path between every
pair of galaxies. What is the sum of these lengths?
*/

#[derive(PartialEq)]
enum Classification {
    Galaxy,
    EmptySpace,
}

struct Location {
    classification: Classification,
    coordinates: (usize, usize),
}

use Classification::*;

fn expand_universe(universe: &mut Vec<Vec<Location>>) {
    let empty_rows = universe.iter().filter(|row| {
        row.iter()
            .all(|location| location.classification == EmptySpace)
    });
    let mut empty_column_indices = Vec::new();
    'columns: for (column_idx, _) in universe[0].iter().enumerate() {
        for (row_idx, _) in universe.iter().enumerate() {
            if universe[row_idx][column_idx].classification == Galaxy {
                continue 'columns;
            }
        }
        empty_column_indices.push(column_idx);
    }
    // TODO
    // - create rows with empty locations (and the expected coordinates) to
    // expand the universe with, and insert them;
    // - insert empty locations in the empty row indices
}

fn shortest_distance(loc_1: &Location, loc_2: &Location) -> usize {
    let ((coord_1_y, coord_1_x), (coord_2_y, coord_2_x)) = (loc_1.coordinates, loc_2.coordinates);
    i32::abs(coord_1_y as i32 - coord_2_y as i32) as usize
        + i32::abs(coord_1_x as i32 - coord_2_x as i32) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_universe() {
        let mut universe = vec![
            vec![
                Location {
                    classification: EmptySpace,
                    coordinates: (0, 0),
                },
                Location {
                    classification: EmptySpace,
                    coordinates: (0, 1),
                },
            ],
            vec![
                Location {
                    classification: EmptySpace,
                    coordinates: (1, 0),
                },
                Location {
                    classification: EmptySpace,
                    coordinates: (1, 1),
                },
            ],
        ];
        expand_universe(&mut universe);
        assert_eq!(universe.len(), 4);
        assert_eq!(universe[0].len(), 4);
    }

    #[test]
    fn test_shortest_distance() {
        let galaxy_1 = Location {
            classification: Galaxy,
            coordinates: (3, 5),
        };
        let galaxy_2 = Location {
            classification: Galaxy,
            coordinates: (0, 0),
        };

        assert_eq!(shortest_distance(&galaxy_1, &galaxy_2), 8);
    }
}
