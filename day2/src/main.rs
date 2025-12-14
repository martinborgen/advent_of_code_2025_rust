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

use std::num::ParseIntError;

fn is_invalid_id(id: u64, verbose: bool) -> bool {
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

fn find_invalid_ids(lower: u64, upper: u64, verbose: bool) -> u64 {
    let mut sum = 0;
    for n in lower..=upper {
        if is_invalid_id(n, verbose) {
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

fn parse_ranges(input: String, verbose: bool) -> u64 {
    let mut sum = 0;
    for case in input.split(',') {
        if case.is_empty() {
            continue;
        }

        let (low, high) = match parse_inputs(case) {
            Ok((low, high)) => (low, high),
            Err(e) => panic!("error parsing int, {}", e),
        };

        let res = find_invalid_ids(low, high, verbose);

        sum += res;
    }
    sum
}

fn main() {
    let input = std::fs::read_to_string("data/input").unwrap();

    let sum = parse_ranges(input, false);
    println!("sum: {}", sum);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_invalid_id_test() {
        assert!(is_invalid_id(11, false));
        assert!(is_invalid_id(22, false));
        assert!(is_invalid_id(1188511885, false));

        assert!(!is_invalid_id(1011, false));
        assert!(!is_invalid_id(1188611885, false));
    }

    #[test]
    fn sample_input() {
        let input = std::fs::read_to_string("data/sample_input").unwrap();

        let sum = parse_ranges(input, true);
        assert_eq!(sum, 1227775554);
    }
}
