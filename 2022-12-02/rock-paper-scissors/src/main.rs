use std::fs;
fn main() {
    let path = "strategy-guide.txt";
    let contents = fs::read_to_string(path).expect("Failed to read file");
    
    println!("Expected guide score: {}", expected_guide_score(&contents));
}

pub fn expected_guide_score(input: &str) -> u32 {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    
    const EXAMPLE_INPUT: &str = indoc! {"
        A Y
        B X
        C Z
    "};

    #[test]
    fn test_expected_guide_score() {
        let result = expected_guide_score(EXAMPLE_INPUT);
        assert_eq!(result, 15)
    }
}
