fn main() {
    println!("Hello, world!");
}

pub fn most_calories_for_an_elf(input: &str) -> u32 {
    let list_for_each_elf: Vec<&str> = input.split("\n\n").collect();
    let calorie_collection_for_each_elf: Vec<Vec<u32>> = list_for_each_elf
        .iter()
        .map(|&s| s.lines().map(|x: &str| x.parse::<u32>().unwrap()).collect())
        .collect();
    println!("{:#?}", calorie_collection_for_each_elf);

    let totals_for_each_elf: Vec<u32> = calorie_collection_for_each_elf
        .iter()
        .map(|c| c.iter().sum::<u32>())
        .collect();
    println!("{:#?}", totals_for_each_elf);

    return *totals_for_each_elf.iter().max().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_most_calories_for_an_elf() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        let result = most_calories_for_an_elf(input);
        assert_eq!(result, 24000)
    }
}
