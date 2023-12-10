/*
--- Day 10: Pipe Maze ---
You use the hang glider to ride the hot air from Desert Island all the way up to
the floating metal island. This island is surprisingly cold and there definitely
aren't any thermals to glide on, so you leave your hang glider behind.

You wander around for a while, but you don't find any people or animals.
However, you do occasionally find signposts labeled "Hot Springs" pointing in a
seemingly consistent direction; maybe you can find someone at the hot springs
and ask them where the desert-machine parts are made.

The landscape here is alien; even the flowers and trees are made of metal. As
you stop to admire some metal grass, you notice something metallic scurry away
in your peripheral vision and jump into a big pipe! It didn't look like any
animal you've ever seen; if you want a better look, you'll need to get ahead of
it.

Scanning the area, you discover that the entire field you're standing on is
densely packed with pipes; it was hard to tell at first because they're the same
metallic silver color as the "ground". You make a quick sketch of all of the
surface pipes you can see (your puzzle input).

The pipes are arranged in a two-dimensional grid of tiles:

| is a vertical pipe connecting north and south.
- is a horizontal pipe connecting east and west.
L is a 90-degree bend connecting north and east.
J is a 90-degree bend connecting north and west.
7 is a 90-degree bend connecting south and west.
F is a 90-degree bend connecting south and east.
. is ground; there is no pipe in this tile.

S is the starting position of the animal; there is a pipe on this tile, but your
sketch doesn't show what shape the pipe has.  Based on the acoustics of the
animal's scurrying, you're confident the pipe that contains the animal is one
large, continuous loop.

For example, here is a square loop of pipe:

.....
.F-7.
.|.|.
.L-J.
.....

If the animal had entered this loop in the northwest corner, the sketch would
instead look like this:

.....
.S-7.
.|.|.
.L-J.
.....

In the above diagram, the S tile is still a 90-degree F bend: you can tell
because of how the adjacent pipes connect to it.

Unfortunately, there are also many pipes that aren't connected to the loop! This
sketch shows the same loop as above:

-L|F7
7S-7|
L|7||
-L-J|
L|-JF

In the above diagram, you can still figure out which pipes form the main loop:
they're the ones connected to S, pipes those pipes connect to, pipes those pipes
connect to, and so on. Every pipe in the main loop connects to its two neighbors
(including S, which will have exactly two pipes connecting to it, and which is
assumed to connect back to those two pipes).

Here is a sketch that contains a slightly more complex main loop:

..F7.
.FJ|.
SJ.L7
|F--J
LJ...

Here's the same example sketch with the extra, non-main-loop pipe tiles also
shown:

7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ

If you want to get out ahead of the animal, you should find the tile in the loop
that is farthest from the starting position. Because the animal is in the pipe,
it doesn't make sense to measure this by direct distance. Instead, you need to
find the tile that would take the longest number of steps along the loop to
reach from the starting point - regardless of which way around the loop the
animal went.

In the first example with the square loop:

.....
.S-7.
.|.|.
.L-J.
.....

You can count the distance each tile in the loop is from the starting point like
this:

.....
.012.
.1.3.
.234.
.....

In this example, the farthest point from the start is 4 steps away.

Here's the more complex loop again:

..F7.
.FJ|.
SJ.L7
|F--J
LJ...

Here are the distances for each tile on that loop:

..45.
.236.
01.78
14567
23...

Find the single giant loop starting at S. How many steps along the loop does it
take to get from the starting position to the point farthest from the starting
position?
*/

// use Description::*;
// use Direction::*;
// use Orientation::*;

// #[derive(Debug, PartialEq)]
// enum Orientation {
//     Horizontal,
//     Vertical,
// }

// #[derive(Debug, PartialEq)]
// enum Direction {
//     East,
//     North,
//     South,
//     West,
// }

// #[derive(Debug, PartialEq)]
// enum Description {
//     Ground,
//     Start,
//     Bend(Direction, Direction),
//     Straight(Orientation),
// }

// #[derive(Debug, PartialEq)]
// struct Position {
//     description: Description,
//     coordinates: (usize, usize),
// }

// struct Sketch<'a> {
//     map: Vec<Vec<Position>>,
//     // (N-S, W-E)
//     start: &'a Position,
//     position: &'a Position,
//     direction: Direction,
// }

// impl<'a> Sketch<'a> {
//     fn choose_direction(m: &Vec<Vec<Position>>, start: &Position) -> Direction {
//         todo!();
//     }

//     fn new(map: Vec<Vec<Position>>, start_coordinates: (usize, usize)) -> Sketch {
//         let start = &map[start_coordinates.0][start_coordinates.1];
//         let direction = choose_direction(&start);
//         Sketch {
//             map,
//             start: &map[start_coordinates.0][start_coordinates.1],
//             position: &map[start_coordinates.0][start_coordinates.1],
//             direction: direction,
//         }
//     }

//     fn next(&'a mut self) {
//         match (&self.position.description, &self.direction) {
//             (Straight(Horizontal), East) => {
//                 self.position =
//                     &self.map[self.position.coordinates.0][self.position.coordinates.1 + 1];
//             }
//             (Straight(Horizontal), West) => {
//                 self.position =
//                     &self.map[self.position.coordinates.0][self.position.coordinates.1 - 1];
//             }
//             (Straight(Vertical), North) => {
//                 self.position =
//                     &self.map[self.position.coordinates.0 - 1][self.position.coordinates.1];
//             }
//             (Straight(Vertical), South) => {
//                 self.position =
//                     &self.map[self.position.coordinates.0 + 1][self.position.coordinates.1]
//             }
//             (Straight(_), _) => panic!("The straight pipe does not support this movement."),
//             (Bend(North, East), South) => {
//                 self.position =
//                     &self.map[self.position.coordinates.0][self.position.coordinates.1 + 1];
//                 self.direction = East;
//             }
//             (Bend(North, East), West) => {
//                 self.position =
//                     &self.map[self.position.coordinates.0 - 1][self.position.coordinates.1];
//                 self.direction = North;
//             }
//             (Bend(North, West), South) => {
//                 self.position =
//                     &self.map[self.position.coordinates.0][self.position.coordinates.1 - 1];
//                 self.direction = West;
//             }
//             (Bend(North, West), East) => {
//                 self.position =
//                     &self.map[self.position.coordinates.0 - 1][self.position.coordinates.1];
//                 self.direction = North;
//             }
//             (Bend(South, East), West) => {
//                 self.position =
//                     &self.map[self.position.coordinates.0 + 1][self.position.coordinates.1];
//                 self.direction = South;
//             }
//             (Bend(South, East), North) => {
//                 self.position =
//                     &self.map[self.position.coordinates.0][self.position.coordinates.1 + 1];
//                 self.direction = East;
//             }
//             (Bend(South, West), East) => {
//                 self.position =
//                     &self.map[self.position.coordinates.0 + 1][self.position.coordinates.1];
//                 self.direction = South;
//             }
//             (Bend(South, West), North) => {
//                 self.position =
//                     &self.map[self.position.coordinates.0][self.position.coordinates.1 - 1];
//                 self.direction = West;
//             }
//             (Bend(_, _), _) => panic!("The bend does not support this movement."),
//             _ => panic!("The movement is not mapped."),
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_parse_input() {
//         let input = b".....\n\
//                                  .S-7.\n\
//                                  .|.|.\n\
//                                  .L-J.\n\
//                                  .....";
//         let expected = vec![
//             vec![Ground, Ground, Ground, Ground, Ground],
//             vec![Ground, Start,              Straight(Horizontal), Bend(South, West),  Ground],
//             vec![Ground, Straight(Vertical), Ground,               Straight(Vertical), Ground],
//             vec![Ground, Bend(North, East),  Straight(Horizontal), Bend(North, West),  Ground],
//             vec![Ground, Ground,             Ground,               Ground,             Ground],
//         ];
//         assert_eq!(parse_input(input), expected);
//     }

//     #[test]
//     fn test_horizontal_move_on_horizontal_pipe() {
//         let sketch_map = vec![vec![
//             Position {
//                 description: Straight(Horizontal),
//                 coordinates: (0, 0),
//             },
//             Position {
//                 description: Ground,
//                 coordinates: (0, 1),
//             },
//         ]];
//         let mut sketch = Sketch {
//             position: &sketch_map[0][0],
//             start: &sketch_map[0][0],
//             map: sketch_map,
//             direction: East,
//         };
//         sketch.next();
//     }

//     #[test]
//     fn test_allowed_move_on_bend() {
//         let initial = Position {
//             description: Bend(South, East),
//             coordinates: (0, 0),
//         };
//         let after_movement = Position {
//             description: Bend(South, West),
//             coordinates: (0, 1),
//         };
//         let sketch = Sketch {
//             map: vec![vec![
//                 initial,
//                 after_movement,
//                 Position {
//                     description: Bend(North, East),
//                     coordinates: (1, 0),
//                 },
//                 Position {
//                     description: Bend(North, West),
//                     coordinates: (1, 1),
//                 },
//             ]],
//             start: &initial,
//             position: &initial,
//             direction: South,
//         };
//         sketch.next();
//         assert_eq!(sketch.position, &after_movement);
//     }
// }
