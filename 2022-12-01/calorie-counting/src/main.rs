use std::fs;
fn main() {
    let path = "elven-list.txt";
    let contents = fs::read_to_string(path).expect("Failed to read file");
    println!("Most calories: {}", most_calories_for_an_elf(&contents));
    println!(
        "Calories for top three elves: {}",
        calories_for_top_three_elves(&contents)
    );
}

pub fn most_calories_for_an_elf(input: &str) -> u32 {
    return *sorted_totals_for_each_elf(input).iter().max().unwrap();
}

fn sorted_totals_for_each_elf(input: &str) -> Vec<u32> {
    let mut totals = calorie_collection_for_each_elf(input)
        .iter()
        .map(|c| c.iter().sum::<u32>())
        .collect::<Vec<u32>>();
    totals.sort();
    return totals;
}

fn calorie_collection_for_each_elf(input: &str) -> Vec<Vec<u32>> {
    input
        .split("\n\n")
        .map(|s| s.lines().map(|x: &str| x.parse::<u32>().unwrap()).collect())
        .collect()
}

pub fn calories_for_top_three_elves(input: &str) -> u32 {
    sorted_totals_for_each_elf(input).iter().rev().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test_most_calories_for_an_elf() {
        let result = most_calories_for_an_elf(EXAMPLE_INPUT);
        assert_eq!(result, 24000)
    }

    #[test]
    fn test_calories_for_top_three_elves() {
        let result = calories_for_top_three_elves(EXAMPLE_INPUT);
        assert_eq!(result, 45000)
    }
}
