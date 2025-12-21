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
        let mut low = 0;
        let mut mid = self.data.len() / 2;
        let mut high = self.data.len() - 1;

        while !self.data[mid].includes(id) && low + 1 != high {
            if id < self.data[mid].lower {
                high = mid;
                mid = (low + high) / 2;
            } else {
                low = mid;
                mid = (low + high) / 2;
            }
        }

        self.data[mid].includes(id)
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
        // database.sort();

        FreshDataBase { data: database }
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

fn count_fresh(database: FreshDataBase, ids: &str) -> u32 {
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
    todo!()
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
        assert_eq!(database.data[1].upper, 14);
        assert_eq!(database.data[2].lower, 12);
        assert_eq!(database.data[2].upper, 18);
        assert_eq!(database.data[3].lower, 16);
        assert_eq!(database.data[3].upper, 20);
    }

    #[test]
    fn test_find_range() {
        let (fresh, ingredients) = read_input("data/sample_input");

        let database = FreshDataBase::read_database(&fresh);

        assert_eq!(database.is_fresh(1), false);
        assert_eq!(database.is_fresh(5), true);
        assert_eq!(database.is_fresh(8), false);
        assert_eq!(database.is_fresh(11), true);
        assert_eq!(database.is_fresh(17), true);
        assert_eq!(database.is_fresh(32), false);
    }

    #[test]
    fn test_sample_input_part_1() {
        let (fresh, ingredients) = read_input("data/sample_input");

        let database = FreshDataBase::read_database(&fresh);

        let fresh_count = count_fresh(database, &ingredients);

        assert_eq!(fresh_count, 3);
    }
}
