fn main() {
    println!("Hello, world!");
}


fn most_calories_for_an_elf(input: &str) -> usize {
    return 0;
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }


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
