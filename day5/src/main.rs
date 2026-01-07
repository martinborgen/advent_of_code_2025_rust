// The Elves in the kitchen explain the situation: because of their complicated new inventory management system, they can't figure out which of their ingredients are fresh and which are spoiled. When you ask how it works, they give you a copy of their database (your puzzle input).

// The database operates on ingredient IDs. It consists of a list of fresh ingredient ID ranges, a blank line, and a list of available ingredient IDs. For example:

// 3-5
// 10-14
// 16-20
// 12-18

// 1
// 5
// 8
// 11
// 17
// 32

// The fresh ID ranges are inclusive: the range 3-5 means that ingredient IDs 3, 4, and 5 are all fresh. The ranges can also overlap; an ingredient ID is fresh if it is in any range.

// The Elves are trying to determine which of the available ingredient IDs are fresh. In this example, this is done as follows:

//     Ingredient ID 1 is spoiled because it does not fall into any range.
//     Ingredient ID 5 is fresh because it falls into range 3-5.
//     Ingredient ID 8 is spoiled.
//     Ingredient ID 11 is fresh because it falls into range 10-14.
//     Ingredient ID 17 is fresh because it falls into range 16-20 as well as range 12-18.
//     Ingredient ID 32 is spoiled.

// So, in this example, 3 of the available ingredient IDs are fresh.

// Process the database file from the new inventory management system. How many of the available ingredient IDs are fresh?

// --- Part Two ---

// The Elves start bringing their spoiled inventory to the trash chute at the back of the kitchen.

// So that they can stop bugging you when they get new inventory, the Elves would like to know all of the IDs that the fresh ingredient ID ranges consider to be fresh. An ingredient ID is still considered fresh if it is in any range.

// Now, the second section of the database (the available ingredient IDs) is irrelevant. Here are the fresh ingredient ID ranges from the above example:

// 3-5
// 10-14
// 16-20
// 12-18

// The ingredient IDs that these ranges consider to be fresh are 3, 4, 5, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, and 20. So, in this example, the fresh ingredient ID ranges consider a total of 14 ingredient IDs to be fresh.

// Process the database file again. How many ingredient IDs are considered to be fresh according to the fresh ingredient ID ranges?

#[derive(Debug, Clone)]
struct FreshRange {
    lower: u64,
    upper: u64,
}

#[derive(Debug, Clone)]
struct FreshDataBase {
    data: Vec<FreshRange>,
}

impl FreshRange {
    fn includes(&self, other: u64) -> bool {
        other >= self.lower && other <= self.upper
    }
}

impl FreshDataBase {
    pub fn is_fresh(&self, id: u64) -> bool {
        for r in self.data.iter() {
            if r.includes(id) {
                return true;
            }
        }

        false
    }

    pub fn read_database(fresh_string: &str) -> FreshDataBase {
        let mut database = Vec::new();

        for line in fresh_string.lines() {
            let (lower, upper) = line.split_once('-').expect("error splitting range");

            database.push(FreshRange {
                lower: lower.parse().expect("error parsing int"),
                upper: upper.parse().expect("error parsing int"),
            });
        }

        database.sort_by(|a, b| a.lower.cmp(&b.lower));

        let mut i = 1;
        while i < database.len() {
            if database[i - 1].upper >= database[i].upper {
                database.remove(i);
            } else if database[i - 1].upper >= database[i].lower {
                database[i - 1].upper = database[i].upper;
                database.remove(i);
            } else {
                i += 1;
            }
        }

        FreshDataBase { data: database }
    }

    pub fn get_total_fresh_count(self) -> u64 {
        let mut count: u64 = 0;

        for fresh_range in self.data {
            let add = fresh_range.upper - fresh_range.lower + 1;
            count += add;
        }

        count
    }
}

fn read_input(filename: &str) -> (String, String) {
    let input_string = std::fs::read_to_string(filename).expect("unable to read file");

    let split_idx = input_string
        .find("\n\n")
        .expect("Couldn't find newline split");

    let (fresh_ranges, ingredients) = input_string.split_at(split_idx);
    (fresh_ranges.to_string(), ingredients.to_string())
}

fn count_fresh(database: &FreshDataBase, ids: &str) -> u32 {
    let mut count = 0;

    for id in ids.lines() {
        if id.is_empty() {
            continue;
        }

        let id_u32 = id.parse().expect("error converting ID to u32");
        if database.is_fresh(id_u32) {
            count += 1;
        }
    }

    count
}

fn main() {
    let (fresh, ingredients) = read_input("data/input");

    let database = FreshDataBase::read_database(&fresh);

    let fresh_count = count_fresh(&database, &ingredients);

    println!("count part 1: {}", fresh_count);

    let total_fresh_possible_part_2 = database.get_total_fresh_count();
    println!("count part 2: {}", total_fresh_possible_part_2);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_import_database() {
        let (fresh, ingredients) = read_input("data/sample_input");

        assert_eq!(fresh, "3-5\n10-14\n16-20\n12-18");
        assert_eq!(ingredients, "\n\n1\n5\n8\n11\n17\n32\n");

        let database = FreshDataBase::read_database(&fresh);
        assert_eq!(database.data[0].lower, 3);
        assert_eq!(database.data[0].upper, 5);
        assert_eq!(database.data[1].lower, 10);
        assert_eq!(database.data[1].upper, 20);
        assert_eq!(database.data.len(), 2);
    }

    #[test]
    fn test_find_range() {
        let (fresh, _ingredients) = read_input("data/sample_input");

        let database = FreshDataBase::read_database(&fresh);

        assert_eq!(database.is_fresh(1), false);
        assert_eq!(database.is_fresh(5), true);
        assert_eq!(database.is_fresh(8), false);
        assert_eq!(database.is_fresh(11), true);
        assert_eq!(database.is_fresh(17), true);
        assert_eq!(database.is_fresh(32), false);

        assert_eq!(database.is_fresh(18), true);
        assert_eq!(database.is_fresh(19), true);
        assert_eq!(database.is_fresh(20), true);
        assert_eq!(database.is_fresh(21), false);
    }

    #[test]
    fn test_sample_input_part_1() {
        let (fresh, ingredients) = read_input("data/sample_input");

        let database = FreshDataBase::read_database(&fresh);

        let fresh_count = count_fresh(&database, &ingredients);

        assert_eq!(fresh_count, 3);
    }

    #[test]
    fn test_sample_input_part_2() {
        let (fresh, _ingredients) = read_input("data/sample_input");

        let database = FreshDataBase::read_database(&fresh);

        let fresh_count = database.get_total_fresh_count();

        assert_eq!(fresh_count, 14);
    }
}
