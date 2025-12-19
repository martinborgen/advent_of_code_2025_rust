// There are batteries nearby that can supply emergency power to the escalator for just such an occasion. The batteries are each labeled with their joltage rating, a value from 1 to 9. You make a note of their joltage ratings (your puzzle input). For example:
//
// 987654321111111
// 811111111111119
// 234234234234278
// 818181911112111
//
// The batteries are arranged into banks; each line of digits in your input corresponds to a single bank of batteries. Within each bank, you need to turn on exactly two batteries; the joltage that the bank produces is equal to the number formed by the digits on the batteries you've turned on. For example, if you have a bank like 12345 and you turn on batteries 2 and 4, the bank would produce 24 jolts. (You cannot rearrange batteries.)
//
// You'll need to find the largest possible joltage each bank can produce. In the above example:
//
//     In 987654321111111, you can make the largest joltage possible, 98, by turning on the first two batteries.
//     In 811111111111119, you can make the largest joltage possible by turning on the batteries labeled 8 and 9, producing 89 jolts.
//     In 234234234234278, you can make 78 by turning on the last two batteries (marked 7 and 8).
//     In 818181911112111, the largest joltage you can produce is 92.
//
// The total output joltage is the sum of the maximum joltage from each bank, so in this example, the total output joltage is 98 + 89 + 78 + 92 = 357.
//
// There are many batteries in front of you. Find the maximum joltage possible from each bank; what is the total output joltage?
//
//

fn find_highest_digit(digit_string: &str, verbose: bool) -> Option<(usize, u32)> {
    let mut highest_index = 0;
    let mut highest: u32 = 0;

    for (i, c) in digit_string[0..digit_string.len()].chars().enumerate() {
        let d = c.to_digit(10)?;

        if d > highest {
            if verbose {
                println!("digit is {}, higher than {}", d, highest);
            }

            highest = d;
            highest_index = i;
        }
    }

    Some((highest_index, highest))
}

fn find_max_joltage_part_1(data: &str, verbose: bool) -> u32 {
    let left_idx;
    let left_digit;
    let right_digit;

    if let Some(left) = find_highest_digit(&data[..data.len() - 1], false) {
        (left_idx, left_digit) = left;
    } else {
        left_digit = 0;
        left_idx = 0;
    }

    if let Some(right) = find_highest_digit(&data[(left_idx + 1_usize)..], false) {
        right_digit = right.1;
    } else {
        right_digit = 0;
    }

    if verbose {
        println!(
            "from {}, largest joltage is {}{}",
            data, left_digit, right_digit
        );
    }

    10 * left_digit + right_digit
}

fn get_total_joltage(data: &str, verbose: bool) -> u32 {
    let mut sum = 0;

    for line in data.lines() {
        if line.is_empty() {
            continue;
        }

        sum += find_max_joltage_part_1(line, true);
    }

    sum
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_highest_digit() {
        let res1 = find_highest_digit("987654321111111", true).unwrap();
        assert_eq!(res1, (0 as usize, 9 as u32));

        let res2 = find_highest_digit("234234234234278", true).unwrap();
        assert_eq!(res2, (13 as usize, 7 as u32));
    }

    #[test]
    fn test_find_max_joltage_part_1() {
        assert_eq!(find_max_joltage_part_1("818181911112111", true), 92);
    }

    #[test]
    fn test_sample_input_part_1() {
        let input = std::fs::read_to_string("data/sample_input").unwrap();
        let res = get_total_joltage(&input, true);

        assert_eq!(res, 357);
    }
}
