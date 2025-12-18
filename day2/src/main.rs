// As it turns out, one of the younger Elves was playing on a gift shop computer and managed to add a whole bunch of invalid product IDs to their gift shop database! Surely, it would be no trouble for you to identify the invalid product IDs for them, right?
//
// They've even checked most of the product ID ranges already; they only have a few product ID ranges (your puzzle input) that you'll need to check. For example:
//
// 11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
// 1698522-1698528,446443-446449,38593856-38593862,565653-565659,
// 824824821-824824827,2121212118-2121212124
//
// (The ID ranges are wrapped here for legibility; in your input, they appear on a single long line.)
//
// The ranges are separated by commas (,); each range gives its first ID and last ID separated by a dash (-).
//
// Since the young Elf was just doing silly patterns, you can find the invalid IDs by looking for any ID which is made only of some sequence of digits repeated twice. So, 55 (5 twice), 6464 (64 twice), and 123123 (123 twice) would all be invalid IDs.
//
// None of the numbers have leading zeroes; 0101 isn't an ID at all. (101 is a valid ID that you would ignore.)
//
// Your job is to find all of the invalid IDs that appear in the given ranges. In the above example:
//
//     11-22 has two invalid IDs, 11 and 22.
//     95-115 has one invalid ID, 99.
//     998-1012 has one invalid ID, 1010.
//     1188511880-1188511890 has one invalid ID, 1188511885.
//     222220-222224 has one invalid ID, 222222.
//     1698522-1698528 contains no invalid IDs.
//     446443-446449 has one invalid ID, 446446.
//     38593856-38593862 has one invalid ID, 38593859.
//     The rest of the ranges contain no invalid IDs.
//
// Adding up all the invalid IDs in this example produces 1227775554.
//
// What do you get if you add up all of the invalid IDs?
//
// --- Part Two ---

// The clerk quickly discovers that there are still invalid IDs in the ranges in your list. Maybe the young Elf was doing other silly patterns as well?
//
// Now, an ID is invalid if it is made only of some sequence of digits repeated at least twice. So, 12341234 (1234 two times), 123123123 (123 three times), 1212121212 (12 five times), and 1111111 (1 seven times) are all invalid IDs.
//
// From the same example as before:
//
//     11-22 still has two invalid IDs, 11 and 22.
//     95-115 now has two invalid IDs, 99 and 111.
//     998-1012 now has two invalid IDs, 999 and 1010.
//     1188511880-1188511890 still has one invalid ID, 1188511885.
//     222220-222224 still has one invalid ID, 222222.
//     1698522-1698528 still contains no invalid IDs.
//     446443-446449 still has one invalid ID, 446446.
//     38593856-38593862 still has one invalid ID, 38593859.
//     565653-565659 now has one invalid ID, 565656.
//     824824821-824824827 now has one invalid ID, 824824824.
//     2121212118-2121212124 now has one invalid ID, 2121212121.
//
// Adding up all the invalid IDs in this example produces 4174379265.
//
// What do you get if you add up all of the invalid IDs using these new rules?

use std::num::ParseIntError;

fn is_invalid_id_part_1(id: u64, verbose: bool) -> bool {
    let id_str = id.to_string();
    let digits = id_str.len();

    if !digits.is_multiple_of(2) {
        return false;
    }

    let left = &id_str[0..(digits / 2)];
    let right = &id_str[(digits / 2)..];

    let is_invalid = left == right;

    if verbose && is_invalid {
        println!("Invalid ID: {}", id);
    }
    is_invalid
}

fn is_invalid_id_part_2(id: u64, verbose: bool) -> bool {
    let id_str = id.to_string();
    let digits = id_str.len();

    for n in 2..=digits {
        let chunk_size = digits / n;
        let id_chunks: Vec<String> = id_str
            .chars()
            .collect::<Vec<_>>()
            .chunks(chunk_size)
            .map(|chunk| chunk.iter().collect())
            .collect();

        if verbose {
            println!("Checking {}, into chunks {:?}", id_str, id_chunks);
        }

        if id_chunks[id_chunks.len() - 1].len() != chunk_size {
            continue;
        }

        let mut invalid = true;

        for chunk in id_chunks[1..].iter() {
            if *chunk != id_chunks[0] {
                invalid = false;
            }
        }

        if invalid {
            if verbose {
                println!(
                    "ID {} is invalid, is {}, {} times",
                    id_str,
                    id_chunks[0],
                    id_chunks.len()
                );
            }
            return true;
        }
    }

    if verbose {
        println!("ID {} is valid", id_str);
    }

    false
}

fn find_invalid_ids(lower: u64, upper: u64, validator: IDValidator, verbose: bool) -> u64 {
    let mut sum = 0;
    for n in lower..=upper {
        if validator(n, verbose) {
            sum += n;
        }
    }
    sum
}

fn parse_inputs(case: &str) -> Result<(u64, u64), ParseIntError> {
    let splitted: Vec<&str> = case.split('-').collect();
    let low = splitted[0].trim().parse()?;
    let high = splitted[1].trim().parse()?;

    Ok((low, high))
}

fn parse_ranges(input: String, validator: IDValidator, verbose: bool) -> u64 {
    let mut sum = 0;
    for case in input.split(',') {
        if case.is_empty() {
            continue;
        }

        let (low, high) = match parse_inputs(case) {
            Ok((low, high)) => (low, high),
            Err(e) => panic!("error parsing int, {}", e),
        };

        let res = find_invalid_ids(low, high, validator, verbose);

        sum += res;
    }
    sum
}

type IDValidator = fn(u64, bool) -> bool;

fn main() {
    let input = std::fs::read_to_string("data/input").unwrap();

    let validator_part_1: IDValidator = is_invalid_id_part_1;
    let validator_part_2: IDValidator = is_invalid_id_part_2;

    let sum_part_1 = parse_ranges(input.clone(), validator_part_1, false);
    println!("sum part 1: {}", sum_part_1);

    let sum_part_2 = parse_ranges(input.clone(), validator_part_2, false);
    println!("sum part 2: {}", sum_part_2);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_invalid_id_test() {
        assert!(is_invalid_id_part_1(11, false));
        assert!(is_invalid_id_part_1(22, false));
        assert!(is_invalid_id_part_1(1188511885, false));

        assert!(!is_invalid_id_part_1(1011, false));
        assert!(!is_invalid_id_part_1(1188611885, false));
    }

    #[test]
    fn sample_input_part_1() {
        let input = std::fs::read_to_string("data/sample_input").unwrap();
        let validator: IDValidator = is_invalid_id_part_1;
        let sum = parse_ranges(input, validator, true);
        assert_eq!(sum, 1227775554);
    }

    #[test]
    fn part_2_invalid_check() {
        assert_eq!(is_invalid_id_part_2(11, true), true);
        assert_eq!(is_invalid_id_part_2(22, true), true);
        assert_eq!(is_invalid_id_part_2(13, true), false);
        assert_eq!(is_invalid_id_part_2(1188511885, true), true);
        assert_eq!(is_invalid_id_part_2(1111111, true), true);
    }

    #[test]
    fn sample_input_part_2() {
        let input = std::fs::read_to_string("data/sample_input").unwrap();
        let validator: IDValidator = is_invalid_id_part_2;
        let sum = parse_ranges(input, validator, true);
        assert_eq!(sum, 4174379265);
    }
}
