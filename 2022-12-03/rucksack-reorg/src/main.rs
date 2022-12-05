use std::fs;
fn main() {
    let path = "input.txt";
    let list = fs::read_to_string(path).expect("Failed to read file");

    println!("Sum of duplicate items priorities: {}", sum_duplicate_items_priorities(&list));
}

pub fn sum_duplicate_items_priorities(input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE_INPUT: &str = indoc! {"
        vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw
    "};

    #[test]
    fn test_sum_duplicate_items_priorities() {
        let result = sum_duplicate_items_priorities(EXAMPLE_INPUT);
        assert_eq!(result, 157)
    }

}