use std::fs;
fn main() {
    let path = "strategy-guide.txt";
    let contents = fs::read_to_string(path).expect("Failed to read file");

    println!("Expected guide score: {}", expected_guide_score_pt1(&contents));
}

pub fn expected_guide_score_pt1(input: &str) -> u32 {
    let rounds = parse_guide_pt1(input);
    println!("rounds: {}", rounds.len());
    rounds.iter()
        .map(|r| r.score())
        .sum()
}

fn parse_guide_pt1(input: &str) -> Vec<Round> {
    input.lines()
        .map(|l: &str| {
            let play1 = Play::from_char(l.chars().nth(0).unwrap());
            let play2 = Play::from_char(l.chars().nth(2).unwrap());
            Round{p1: play1, p2: play2}
        })
        .collect()
}

#[derive(PartialEq, Debug)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

#[derive(PartialEq, Debug)]
enum Result {
    Win,
    Draw,
    Loss,
}

impl Play {
    fn from_char(c: char) -> Play {
        match c {
            'A' | 'X' => Play::Rock,
            'B' | 'Y' => Play::Paper,
            'C' | 'Z' => Play::Scissors,
            _ => unreachable!()
        }
    }

    fn against(&self, other: &Play) -> Result {
        if *self == *other {
            return Result::Draw;
        }
        else {
            match(self, other) {
                (Play::Rock, Play::Scissors) => Result::Win,
                (Play::Scissors, Play::Paper) => Result::Win,
                (Play::Paper, Play::Rock) => Result::Win,
                _ => Result::Loss
            }
        }
    }
}

#[derive(Debug, PartialEq)]
struct Round {
    p1: Play,
    p2: Play
}

impl Round {
    fn score(&self) -> u32 {
        self.shape_score() + self.outcome_score()
    }

    fn shape_score(&self) -> u32 {
        match self.p2 {
            Play::Rock => 1,
            Play::Paper => 2,
            Play::Scissors => 3,
        }
    }

    fn outcome_score(&self) -> u32 {
        match self.p2.against(&self.p1) {
            Result::Loss => 0,
            Result::Draw => 3,
            Result::Win => 6,
        }
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
        let result = expected_guide_score_pt1(EXAMPLE_INPUT);
        assert_eq!(result, 15)
    }

    #[test]
    fn test_parse_input() {
        let result: Vec<Round> = parse_guide_pt1(EXAMPLE_INPUT);
        assert_eq!(
            result,
            vec![
                Round{p1: Play::Rock, p2: Play::Paper},
                Round{p1: Play::Paper, p2: Play::Rock},
                Round{p1: Play::Scissors, p2: Play::Scissors}
            ]
        )
    }

    #[test]
    fn test_shape_score() {
        assert_eq!(Round{p1: Play::Rock,     p2: Play::Paper    }.shape_score(), 2);
        assert_eq!(Round{p1: Play::Paper,    p2: Play::Paper    }.shape_score(), 2);
        assert_eq!(Round{p1: Play::Scissors, p2: Play::Paper    }.shape_score(), 2);

        assert_eq!(Round{p1: Play::Paper,    p2: Play::Rock     }.shape_score(), 1);
        assert_eq!(Round{p1: Play::Paper,    p2: Play::Paper    }.shape_score(), 2);
        assert_eq!(Round{p1: Play::Paper,    p2: Play::Scissors }.shape_score(), 3);
    }

    #[test]
    fn test_outcome_score() {
        assert_eq!(Round{p1: Play::Rock,     p2: Play::Paper    }.outcome_score(), 6);
        assert_eq!(Round{p1: Play::Paper,    p2: Play::Paper    }.outcome_score(), 3);
        assert_eq!(Round{p1: Play::Scissors, p2: Play::Paper    }.outcome_score(), 0);

        assert_eq!(Round{p1: Play::Paper,    p2: Play::Rock     }.outcome_score(), 0);
        assert_eq!(Round{p1: Play::Paper,    p2: Play::Paper    }.outcome_score(), 3);
        assert_eq!(Round{p1: Play::Paper,    p2: Play::Scissors }.outcome_score(), 6);
    }

    #[test]
    fn test_score() {
        assert_eq!(Round{p1: Play::Rock,     p2: Play::Paper    }.score(), 8);
        assert_eq!(Round{p1: Play::Paper,    p2: Play::Paper    }.score(), 5);
        assert_eq!(Round{p1: Play::Scissors, p2: Play::Paper    }.score(), 2);

        assert_eq!(Round{p1: Play::Paper,    p2: Play::Rock     }.score(), 1);
        assert_eq!(Round{p1: Play::Paper,    p2: Play::Paper    }.score(), 5);
        assert_eq!(Round{p1: Play::Paper,    p2: Play::Scissors }.score(), 9);
    }

    #[test]
    fn test_play() {
        assert_eq!(Play::Rock.against(&Play::Paper), Result::Loss);
        assert_eq!(Play::Rock.against(&Play::Rock), Result::Draw);
        assert_eq!(Play::Rock.against(&Play::Scissors), Result::Win);
    }
}
