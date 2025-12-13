// https://adventofcode.com/2025/day/1
//
// The attached document (your puzzle input) contains a sequence of rotations, one per line, which tell you how to open the safe. A rotation starts with an L or R which indicates whether the rotation should be to the left (toward lower numbers) or to the right (toward higher numbers). Then, the rotation has a distance value which indicates how many clicks the dial should be rotated in that direction.

// So, if the dial were pointing at 11, a rotation of R8 would cause the dial to point at 19. After that, a rotation of L19 would cause it to point at 0.
//
// Because the dial is a circle, turning the dial left from 0 one click makes it point at 99. Similarly, turning the dial right from 99 one click makes it point at 0.
//
// So, if the dial were pointing at 5, a rotation of L10 would cause it to point at 95. After that, a rotation of R5 could cause it to point at 0.
//
// The dial starts by pointing at 50.
//
// You could follow the instructions, but your recent required official North Pole secret entrance security training seminar taught you that the safe is actually a decoy. The actual password is the number of times the dial is left pointing at 0 after any rotation in the sequence.
//
// For example, suppose the attached document contained the following rotations:
//
// L68
// L30
// R48
// L5
// R60
// L55
// L1
// L99
// R14
// L82
//
// Following these rotations would cause the dial to move as follows:
//
//     The dial starts by pointing at 50.
//     The dial is rotated L68 to point at 82.
//     The dial is rotated L30 to point at 52.
//     The dial is rotated R48 to point at 0.
//     The dial is rotated L5 to point at 95.
//     The dial is rotated R60 to point at 55.
//     The dial is rotated L55 to point at 0.
//     The dial is rotated L1 to point at 99.
//     The dial is rotated L99 to point at 0.
//     The dial is rotated R14 to point at 14.
//     The dial is rotated L82 to point at 32.
//
// Because the dial points at 0 a total of three times during this process, the password in this example is 3.
//

use std::fs;

fn count_zero_positions(input: String, verbose: bool) -> i32 {
    let mut times_at_zero = 0;
    let mut dial_pos = 50;

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let dir = if line.starts_with('L') {
            -1
        } else if line.starts_with('R') {
            1
        } else {
            panic!("Unsupported input: {}", line);
        };

        let rotation: i32 = line[1..].parse().unwrap();

        if verbose {
            println!("Dial pos: {}, Rotation: {}", dial_pos, dir * rotation);
        }

        dial_pos = (dial_pos + dir * rotation) % 100;
        if dial_pos < 0 {
            dial_pos = (dial_pos + 100) % 100;
        }

        if dial_pos == 0 {
            times_at_zero += 1;
        }
    }

    times_at_zero
}

fn main() {
    let input = match fs::read_to_string("data/input") {
        Ok(input) => input,
        Err(err) => panic!("File not found {}", err),
    };

    let dial_pos = count_zero_positions(input, false);

    println!("dial pos: {}", dial_pos);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_input() {
        let input = match fs::read_to_string("data/example_input") {
            Ok(input) => input,
            Err(err) => panic!("File not found {}", err),
        };

        let dial_pos = count_zero_positions(input, true);

        assert_eq!(dial_pos, 3);
    }
}
