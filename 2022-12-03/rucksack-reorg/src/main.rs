use std::collections::HashSet;
use std::fs;

fn main() {
    let path = "input.txt";
    let list = fs::read_to_string(path).expect("Failed to read file");

    println!(
        "Sum of duplicate items priorities: {}",
        sum_duplicate_items_priorities(&list)
    );
}

pub fn sum_duplicate_items_priorities(input: &str) -> u32 {
    return duplicate_items(input).iter().map(|c| priority(*c)).sum();
}

fn duplicate_items(input: &str) -> Vec<char> {
    let backpacks: Vec<Backpack> = parse_input(input);
    return backpacks.iter().map(|b| b.duplicate_item()).collect();
}

fn priority(c: char) -> u32 {
    let code = c as u32;
    match code {
        65..=90 => code - 64 + 26,
        97..=122 => code - 96,
        _ => unreachable!(),
    }
}

fn parse_input(input: &str) -> Vec<Backpack> {
    input.lines().map(|l: &str| parse_backpack(l)).collect()
}

fn parse_backpack(line: &str) -> Backpack {
    assert!(
        line.len() % 2 == 0,
        "backpack should contain even number of items"
    );
    let first_compartment = &line[..line.len() / 2];
    let second_compartment = &line[line.len() / 2..];
    assert!(
        first_compartment.len() == second_compartment.len(),
        "compartments should be equally sized"
    );
    let first_compartment_chars: Vec<char> = first_compartment.chars().collect();
    let first_compartment_char_set: HashSet<char> = first_compartment_chars.into_iter().collect();
    let second_compartment_chars: Vec<char> = second_compartment.chars().collect();
    let second_compartment_char_set: HashSet<char> = second_compartment_chars.into_iter().collect();

    (first_compartment_char_set, second_compartment_char_set)
}

type Backpack = (HashSet<char>, HashSet<char>);

trait Duplicitous {
    fn duplicate_item(&self) -> char;
}
impl Duplicitous for Backpack {
    fn duplicate_item(&self) -> char {
        let duplicates: HashSet<char> = (self.0).intersection(&self.1).copied().collect();
        assert!(duplicates.len() == 1, "Backpack expected to contain one and only one duplicate");
        return *duplicates.iter().next().unwrap();
    }
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

    #[test]
    fn test_duplicate_items() {
        assert_eq!(
            duplicate_items(EXAMPLE_INPUT),
            vec!['p', 'L', 'P', 'v', 't', 's']
        )
    }

    #[test]
    fn test_parse_input() {
        let result: Vec<Backpack> = parse_input(EXAMPLE_INPUT);
        // let first_compartment_chars: Vec<char> = "vJrwpWtwJgWr".chars().collect();
        // let first_compartment_char_set: HashSet<char> = first_compartment_chars.into_iter().collect();
        let first_compartment_char_set: HashSet<char> =
            HashSet::from(['v', 'J', 'r', 'w', 'p', 'W', 't', 'w', 'J', 'g', 'W', 'r']);
        // let second_compartment_chars: Vec<char> = "hcsFMMfFFhFp".chars().collect();
        // let second_compartment_char_set: HashSet<char> = second_compartment_chars.into_iter().collect();
        let second_compartment_char_set: HashSet<char> =
            HashSet::from(['h', 'c', 's', 'F', 'M', 'M', 'f', 'F', 'F', 'h', 'F', 'p']);
        assert_eq!(
            result[0],
            (first_compartment_char_set, second_compartment_char_set)
        );
        assert_eq!(result.len(), 6);
    }

    #[test]
    fn test_duplicate_item() {
        let backpack: Backpack = (
            HashSet::from(['a', 'b', 'c']),
            HashSet::from(['c', 'd', 'e'])
        );
        assert_eq!(backpack.duplicate_item(), 'c');
    }
}
