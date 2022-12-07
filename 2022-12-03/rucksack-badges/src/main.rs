use std::collections::HashSet;
use std::fs;

fn main() {
    let path = "input.txt";
    let list = fs::read_to_string(path).expect("Failed to read file");

    println!(
        "Sum of duplicate items priorities: {}",
        sum_badge_priorities(&list)
    );
}

pub fn sum_badge_priorities(input: &str) -> u32 {
    return badges(input).iter().map(|c| priority(*c)).sum();
}

fn badges(input: &str) -> Vec<char> {
    let groups: Vec<Group> = parse_input_to_groups(input);
    return groups.iter().map(|group| group.badge()).collect();
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

fn parse_input_to_groups(input: &str) -> Vec<Group> {
    return parse_input(input).chunks(3).map(|x| (x[0],x[1],x[2])).collect();
}

fn parse_backpack(line: &str) -> Backpack {
    line.chars().into_iter().collect()
}

type Backpack = HashSet<char>;
type Group = (Backpack, Backpack, Backpack);

trait Badged {
    fn badge(&self) -> char;
}
impl Badged for Group {
    fn badge(&self) -> char {
        let commonFirstAndSecond: HashSet<char> = (self.0).intersection(&self.1).copied().collect();
        let common: HashSet<char> = (commonFirstAndSecond).intersection(&self.2).copied().collect();
        assert!(common.len() == 1, "Group expected to contain one and only one badge");
        return *common.iter().next().unwrap();
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
    fn test_sum_badge_priorities() {
        let result = sum_badge_priorities(EXAMPLE_INPUT);
        assert_eq!(result, 70)
    }

    #[test]
    fn test_parse_input() {
        let result: Vec<Backpack> = parse_input(EXAMPLE_INPUT);
        let first_backpack_char_set: HashSet<char> =
            HashSet::from(['v','J','r','w','p','W','t','w','J','g','W','r','h','c','s','F','M','M','f','F','F','h','F','p']);
        assert_eq!(
            result[0],
            first_backpack_char_set
        );
        assert_eq!(result.len(), 6);
    }

    #[test]
    fn test_parse_input_to_groups() {
        let result: Vec<Group> = parse_input_to_groups(EXAMPLE_INPUT);
        assert_eq!(result.len(), 2);
    }
}
