// If you can optimize the work the forklifts are doing, maybe they would have time to spare to break through the wall.
//
// The rolls of paper (@) are arranged on a large grid; the Elves even have a helpful diagram (your puzzle input) indicating where everything is located.
//
// For example:
//
// ..@@.@@@@.
// @@@.@.@.@@
// @@@@@.@.@@
// @.@@@@..@.
// @@.@@@@.@@
// .@@@@@@@.@
// .@.@.@.@@@
// @.@@@.@@@@
// .@@@@@@@@.
// @.@.@@@.@.
//
// The forklifts can only access a roll of paper if there are fewer than four rolls of paper in the eight adjacent positions. If you can figure out which rolls of paper the forklifts can access, they'll spend less time looking and more time breaking down the wall to the cafeteria.
//
// In this example, there are 13 rolls of paper that can be accessed by a forklift (marked with x):
//
// ..xx.xx@x.
// x@@.@.@.@@
// @@@@@.x.@@
// @.@@@@..@.
// x@.@@@@.@x
// .@@@@@@@.@
// .@.@.@.@@@
// x.@@@.@@@@
// .@@@@@@@@.
// x.x.@@@.x.
//
// Consider your complete diagram of the paper roll locations. How many rolls of paper can be accessed by a forklift?
//
//

fn read_board(filename: &str) -> Result<Vec<Vec<char>>, Box<dyn std::error::Error>> {
    let f = std::fs::read_to_string(filename)?;

    let out = f
        .lines()
        .filter(|line| line.len() > 0)
        .map(|line| line.chars().collect())
        .collect();

    Ok(out)
}

fn count_roll_neighbours(board: &[Vec<char>], r: usize, c: usize, verbose: bool) -> u32 {
    let min_rows = i32::max(0, r as i32 - 1);
    let min_cols = i32::max(0, c as i32 - 1);

    let max_rows = usize::min(board.len() - 1, r + 1);
    let max_cols = usize::min(board[0].len() - 1, c + 1);

    let rows = min_rows as usize..=max_rows;
    let cols = min_cols as usize..=max_cols;

    let mut count = 0;
    for row in &board[rows] {
        for c in &row[cols.clone()] {
            if verbose {
                println!("comparing {} and '@', comparision is {}", *c, *c == '@');
            }

            if *c == '@' {
                count += 1;
            }
        }
    }

    if verbose {
        println!("{}:{} has {} neighboring rolls", r, c, count);
        println!("range rows: [{}, {})", min_rows, max_rows);
        println!("range cols: [{}, {})", min_cols, max_cols);
    }
    count - 1
}

fn find_loose_rolls(board: Vec<Vec<char>>, verbose: bool) -> u32 {
    let mut count = 0;

    for (r, row) in board.iter().enumerate() {
        for (c, _) in row.iter().enumerate() {
            let neighbour_count = count_roll_neighbours(&board, r, c, false);
            if neighbour_count < 4 && board[r][c] == '@' {
                count += 1;
            }

            if verbose {
                println!("{}:{} has {} neighbors", r, c, count);
            }
        }
    }

    count
}

fn main() {
    let tmp = read_board("data/sample_input");
    println!("{:?}", tmp);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count_loose_rolls() {
        let input = read_board("data/sample_input").unwrap();
        let res1 = count_roll_neighbours(&input, 0, 2, true);
        let res2 = count_roll_neighbours(&input, 1, 2, true);
        let res3 = count_roll_neighbours(&input, 9, 8, true);

        assert_eq!(res1, 3);
        assert_eq!(res2, 6);
        assert_eq!(res3, 2);
    }

    #[test]
    fn test_sample_input_part_1() {
        let input = read_board("data/sample_input").unwrap();
        let count = find_loose_rolls(input, true);
        assert_eq!(count, 13);
    }
}
