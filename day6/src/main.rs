fn transpose_input_part_1(input: String) -> Vec<Vec<String>> {
    let input_array: Vec<Vec<String>> = input
        .lines()
        .map(|line| line.split_whitespace().map(|s| s.to_string()).collect())
        .collect();

    let mut out: Vec<Vec<String>> = Vec::new();
    for col in 0..input_array[0].len() {
        let mut column = Vec::new();
        for row in 0..input_array.len() {
            column.push(input_array[row][col].clone());
        }
        out.push(column);
    }

    out
}

fn compute_part_1(input: &[String]) -> u64 {
    assert!(!input.is_empty());
    let operator = input
        .last()
        .expect("could not get operator, i.e. last element")
        .chars()
        .last()
        .unwrap();
    let nums: Vec<u64> = input[..(input.len() - 1)]
        .iter()
        .map(|s| s.parse().expect("error parsing int"))
        .collect();

    match operator {
        '+' => nums.iter().sum(),
        '*' => nums.iter().product(),
        _ => panic!("Invalid operator!"),
    }
}

fn main() {
    let raw_input = std::fs::read_to_string("data/input").expect("Error reading input");

    let input = transpose_input_part_1(raw_input);

    let mut res_part_1 = 0;
    for i in input {
        res_part_1 += compute_part_1(&i);
    }

    println!("result part 1: {}", res_part_1);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample_input() {
        let raw_input = std::fs::read_to_string("data/sample_input").expect("Error reading input");

        let input = transpose_input_part_1(raw_input);
        assert!(!input.is_empty());

        let res1 = compute_part_1(&input[0]);
        let res2 = compute_part_1(&input[1]);
        let res3 = compute_part_1(&input[2]);
        let res4 = compute_part_1(&input[3]);

        assert_eq!(res1, 33210);
        assert_eq!(res2, 490);
        assert_eq!(res3, 4243455);
        assert_eq!(res4, 401);

        assert_eq!(res1 + res2 + res3 + res4, 4277556);
    }
}
