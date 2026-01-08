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

fn read_input_part_2(path: &str) -> Vec<Vec<char>> {
    let out: Vec<Vec<char>> = std::fs::read_to_string(path)
        .expect("error reading file")
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    out
}

fn transpose_input_part_2(input: Vec<Vec<char>>) -> Vec<Vec<String>> {
    let mut out: Vec<Vec<String>> = Vec::new();
    let rows: usize = input.len();
    let cols: usize = input[0].len();

    let mut i = 0;
    let mut operator = input[rows - 1][i];
    let mut out_len;
    out.push(Vec::new());
    while i < cols {
        let mut num = String::new();
        for c in input.iter().filter(|c| c[i].is_numeric()) {
            num.push(c[i]);
        }

        out_len = out.len();
        if num.is_empty() {
            out[out_len - 1].push(operator.to_string());
            i += 1;
            operator = input[rows - 1][i];
            out.push(Vec::new());
        } else {
            out[out_len - 1].push(num);
            i += 1;
        }
    }

    out_len = out.len();
    out[out_len - 1].push(operator.to_string());
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
        _ => panic!("Invalid operator: {}", operator),
    }
}

fn main() {
    let input_path = "data/input";
    let raw_input = std::fs::read_to_string(input_path).expect("Error reading input");

    let input_part_1 = transpose_input_part_1(raw_input);

    let mut res_part_1 = 0;
    for i in input_part_1 {
        res_part_1 += compute_part_1(&i);
    }

    println!("result part 1: {}", res_part_1);

    let input_part_2 = transpose_input_part_2(read_input_part_2(input_path));
    let mut res_part_2 = 0;
    for i in input_part_2 {
        res_part_2 += compute_part_1(&i);
    }

    println!("result part 2: {}", res_part_2);
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

    #[test]
    fn test_sample_input_part_2() {
        let input_part_2 = transpose_input_part_2(read_input_part_2("data/sample_input"));
        let mut res_part_2 = 0;
        for i in input_part_2 {
            res_part_2 += compute_part_1(&i);
        }

        assert_eq!(res_part_2, 3263827);
    }
}
