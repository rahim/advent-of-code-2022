fn main() {
    println!("Hello, world!");
}


pub fn most_calories_for_an_elf(input: &str) -> usize {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_most_calories_for_an_elf() {
        let input =
"1000
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
        assert_eq!(result, 2400)
    }
}
