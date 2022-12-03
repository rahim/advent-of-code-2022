use std::fs;
fn main() {
    let path = "strategy-guide.txt";
    let contents = fs::read_to_string(path).expect("Failed to read file");

    println!("Expected guide score: {}", expected_guide_score(&contents));
}

pub fn expected_guide_score(input: &str) -> u32 {
    return 0;
}

enum Play {
    Rock,
    Paper,
    Scissors,
}

struct Round {
    p1: Play,
    p2: Play
}

impl Round {
    fn score(&self) -> u8 {
        0
    }

    fn shape_score(&self) -> u8 {
        match self.p1 {
            Play::Rock => 1,
            Play::Paper => 2,
            Play::Scissors => 3,
        }
    }

    fn outcome_score(&self) -> u8 {
        0
    }
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

    #[test]
    fn test_shape_score() {
        assert_eq!(Round{p1: Play::Rock,     p2: Play::Paper}.shape_score(), 1);
        assert_eq!(Round{p1: Play::Paper,    p2: Play::Paper}.shape_score(), 2);
        assert_eq!(Round{p1: Play::Scissors, p2: Play::Paper}.shape_score(), 3);
    }

    #[test]
    fn test_outcome_score() {
        assert_eq!(Round{p1: Play::Rock,     p2: Play::Paper}.outcome_score(), 0);
        assert_eq!(Round{p1: Play::Paper,    p2: Play::Paper}.outcome_score(), 3);
        assert_eq!(Round{p1: Play::Scissors, p2: Play::Paper}.outcome_score(), 6);
    }
}
