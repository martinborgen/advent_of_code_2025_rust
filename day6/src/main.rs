fn transpose_input(input: String) -> Vec<Vec<String>> {
    let input: Vec<Vec<String>> = input
        .lines()
        .map(|line| line.split_whitespace().map(|s| s.to_string()).collect())
        .collect();

    let mut out: Vec<Vec<String>> = Vec::new();
    for i in 0..input.len() {
        assert_eq!(input.len(), input[i].len());

        out.push(Vec::new());
        for j in 0..input[i].len() {
            out[i].push(input[j][i].clone());
        }
    }

    out
}

fn compute(input: &[String]) -> u64 {
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

    let input = transpose_input(raw_input);

    let mut res_part_1 = 0;
    for i in input {
        res_part_1 += compute(&i);
    }

    println!("result part 1: {}", res_part_1);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample_input() {
        let raw_input = std::fs::read_to_string("data/sample_input").expect("Error reading input");

        let input = transpose_input(raw_input);
        assert!(!input.is_empty());

        let res1 = compute(&input[0]);
        let res2 = compute(&input[1]);
        let res3 = compute(&input[2]);
        let res4 = compute(&input[3]);

        assert_eq!(res1, 33210);
        assert_eq!(res2, 490);
        assert_eq!(res3, 4243455);
        assert_eq!(res4, 401);

        assert_eq!(res1 + res2 + res3 + res4, 4277556);
    }
}
