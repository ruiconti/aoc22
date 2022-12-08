use std::fs::read_to_string;
fn main() {
    let input_filepath = "./input/strategy_guide.txt";
    // reads the input string into the game
    let rounds_raw = read_to_string(input_filepath).expect(format!("Unable to read {}", input_filepath).as_str());

    for part in vec![Problem::Part1, Problem::Part2] {
        let game = RockPaperScissors::new(rounds_raw.as_str(), part);
        let outcome = game.run();

        println!("Game for {:?}: {:?}", part, outcome);
    }
}

// The game is rock, paper, scissors. It is always played by 2 fighting hands and 2 fighting hands only.
// Each round can only have one winning hand. And to tell the winner, we need to compare both hands.
// 
// Note: Hand and outcome scores _are not_ dependent. For example, by choosing scissors you are _guaranteed_ to earn 3 points.
//
// **Gameplay**:
// The game is played by 2 players —myself and the opponent. Each player chooses a hand and the winner is determined by the rules above.
// We'll use a encrypted strategy guide that tells us what hand myself and the opponent will choose for each round.
// The game is played in rounds. Each line of the guide will define the hand for each player for that round.
// The hand for each player will be defined by a word. Each player will have a set of words to define their hand.
//
// Myself hands:
// - Rock: X
// - Paper: Y
// - Scissors: Z
//
// Opponent words:
// - Rock: A
// - Paper: B
// - Scissors: C
// 
// So a round example will be in the form of:
// A Y
// 
// Meaning that the opponent chose rock (A) and myself paper (Y). 
// Round winner: myself.
// Round scores:
//  Opponent: 0 + 1 = 1
//  Myself: 6 + 2 = 8

#[derive(Debug, Clone, Copy)]
enum Problem {
    Part1,
    Part2
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Hand {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Debug)]
#[derive(PartialEq)]
enum Outcome {
    MyselfWins,
    OpponentWins,
    Draw
}

#[derive(Debug)]
struct RoundSetup {
    opponent: Hand,
    myself: Hand,
}

#[derive(Debug)]
struct RoundOutcome {
    winner: Outcome,
    score_opponent: i32,
    score_myself: i32
}

struct RockPaperScissors {
    rounds_setup: Vec<RoundSetup>,
}

// **Winning rules**:
// Rock     & scissors -> Winner: Rock
// Scissors & paper    -> Winner: Scissors
// Paper    & rock     -> Winner: Paper
//
// Diagonal is always a draw:
// Rock     & rock     -> Draw
// Scissors & scissors -> Draw
// Paper    & paper    -> Draw
impl std::cmp::PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Hand::Rock, Hand::Rock) => Some(std::cmp::Ordering::Equal),
            (Hand::Scissors, Hand::Scissors) => Some(std::cmp::Ordering::Equal),
            (Hand::Paper, Hand::Paper) => Some(std::cmp::Ordering::Equal),
            (Hand::Rock, Hand::Scissors) => Some(std::cmp::Ordering::Greater),
            (Hand::Scissors, Hand::Paper) => Some(std::cmp::Ordering::Greater),
            (Hand::Paper, Hand::Rock) => Some(std::cmp::Ordering::Greater),
            (Hand::Rock, Hand::Paper) => Some(std::cmp::Ordering::Less),
            (Hand::Scissors, Hand::Rock) => Some(std::cmp::Ordering::Less),
            (Hand::Paper, Hand::Scissors) => Some(std::cmp::Ordering::Less),
        }
    }
}

trait Game {
    fn new(rounds: &str, problem: Problem) -> Self;
    fn run(&self) -> RoundOutcome;
}

impl Game for RockPaperScissors {
    fn new(rounds: &str, problem: Problem) -> Self {
        let mut rounds_setup = Vec::new();
        for line in rounds.lines() {
            let sanitized_line = line.trim();
            if sanitized_line.is_empty() {
                continue;
            }

            rounds_setup.push(RoundSetup::new(sanitized_line, problem));
        }
        Self { rounds_setup }
    }

    fn run(&self) -> RoundOutcome {
        let mut game_outcome = RoundOutcome { winner: Outcome::Draw, score_opponent: 0, score_myself: 0 };
        for round in &self.rounds_setup {
            let round_outcome = round.play();
            game_outcome.score_opponent += round_outcome.score_opponent;
            game_outcome.score_myself += round_outcome.score_myself;
        }

        game_outcome.winner = if game_outcome.score_myself == game_outcome.score_opponent {
            Outcome::Draw
        } else if game_outcome.score_myself > game_outcome.score_opponent {
            Outcome::MyselfWins
        } else {
            Outcome::OpponentWins
        };
        return game_outcome;
    }

}

impl RoundSetup {
    fn new(line: &str, problem: Problem) -> Self {
        let mut words = line.split_whitespace();
        let opponent = match words.next() {
            Some("A") => Hand::Rock,
            Some("B") => Hand::Paper,
            Some("C") => Hand::Scissors,
            _ => panic!("Invalid opponent hand")
        };

        // The way that we handle the second column i.e. `myself` differs 
        // depending on which part of the problem we're at.
        let myself = match problem {
            // First part: we inferred that X, Y and Z were mappings to hands.
            Problem::Part1 => 
                match words.next() {
                    Some("X") => Hand::Rock,
                    Some("Y") => Hand::Paper,
                    Some("Z") => Hand::Scissors,
                    _ => panic!("Invalid myself hand")
                },
            // Second part: we know that X, Y and Z are mappings to outcomes that
            // depends on the opponent's hand.
            Problem::Part2 => match words.next() {
                    Some("X") /* I need to lose */ => { 
                        match opponent {
                            Hand::Rock => Hand::Scissors,
                            Hand::Paper => Hand::Rock,
                            Hand::Scissors => Hand::Paper,
                        }
                    },
                    Some("Y") /* I need to draw */ => {
                        match opponent {
                            Hand::Rock => Hand::Rock,
                            Hand::Paper => Hand::Paper,
                            Hand::Scissors => Hand::Scissors,
                        }
                    },
                    Some("Z") /* I need to win */ => {
                        match opponent {
                            Hand::Rock => Hand::Paper,
                            Hand::Paper => Hand::Scissors,
                            Hand::Scissors => Hand::Rock,
                        }
                    },
                    _ => panic!("Invalid word.")
            },
        };
        Self { opponent: opponent, myself: myself }
    }

    fn play(&self) -> RoundOutcome {
        let winner = match self.opponent.partial_cmp(&self.myself) {
            Some(std::cmp::Ordering::Equal) => Outcome::Draw,
            Some(std::cmp::Ordering::Greater) => Outcome::OpponentWins,
            Some(std::cmp::Ordering::Less) => Outcome::MyselfWins,
            None => panic!("Invalid round")
        };

        let score_opponent = match winner {
            Outcome::OpponentWins => self.opponent as i32  + 6,
            Outcome::Draw => self.opponent as i32 + 3,
            Outcome::MyselfWins => self.opponent as i32,
        };

        let score_myself = match winner {
            Outcome::MyselfWins => self.myself as i32 + 6,
            Outcome::Draw => self.myself as i32 + 3,
            Outcome::OpponentWins => self.myself as i32,
        };

        return RoundOutcome { winner, score_opponent, score_myself };
    }
}

// **Scoring rules**:
// Outcome score:
// Win:  6 points
// Draw: 3 points
// Loss: 0 points
// 
// Hand score:
// Rock:     1 point
// Paper:    2 points
// Scissors: 3 points

#[cfg(test)]
mod test_game {
    use super::*;

    #[test]
    fn part2_draw() {
        // Arrange
        let rounds = "A X
        B Y
        C Z";
        let game = RockPaperScissors::new(rounds, Problem::Part2);

        // Act
        let outcome = game.run();

        // Assert
        assert_eq!(outcome.winner, Outcome::Draw);
        assert_eq!(outcome.score_myself, 3 + 5 + 7);
        assert_eq!(outcome.score_opponent, 7 + 5 + 3);
    }

    #[test]
    fn part1_myself_wins() {
        // Arrange
        let rounds = "B Z
        B Y
        C Z";
        let game = RockPaperScissors::new(rounds, Problem::Part1);

        // Act
        let outcome = game.run();

        // Assert
        assert_eq!(outcome.winner, Outcome::MyselfWins);
        assert_eq!(outcome.score_myself, 9 + 5 + 6);
        assert_eq!(outcome.score_opponent, 2 + 5 + 6);
    }

    #[test]
    fn part1_draw() {
        // Arrange
        let rounds = "A X
        B Y
        C Z";
        let game = RockPaperScissors::new(rounds, Problem::Part1);

        // Act
        let outcome = game.run();

        // Assert
        assert_eq!(outcome.winner, Outcome::Draw);
        assert_eq!(outcome.score_myself, 4 + 5 + 6);
        assert_eq!(outcome.score_opponent, 4 + 5 + 6);
    }

    #[test]
    fn part1_opponent_wins() {
        // Arrange
        let rounds = "B X
        B Y
        C Z";
        let game = RockPaperScissors::new(rounds, Problem::Part1);

        // Act
        let outcome = game.run();

        // Assert
        assert_eq!(outcome.winner, Outcome::OpponentWins);
        assert_eq!(outcome.score_myself, 1 + 5 + 6);
        assert_eq!(outcome.score_opponent, 8 + 5 + 6);
    }

}


#[cfg(test)]
mod test_round_setup {
    use super::*;

    #[test]
    fn test_round_setup_match() {
        // Arrange
        let round_setups = vec![
            RoundSetup{ opponent: Hand::Rock, myself: Hand::Rock },
            RoundSetup{ opponent: Hand::Rock, myself: Hand::Paper },
            RoundSetup{ opponent: Hand::Rock, myself: Hand::Scissors },
            RoundSetup{ opponent: Hand::Paper, myself: Hand::Rock },
            RoundSetup{ opponent: Hand::Paper, myself: Hand::Paper },
            RoundSetup{ opponent: Hand::Paper, myself: Hand::Scissors },
            RoundSetup{ opponent: Hand::Scissors, myself: Hand::Rock },
            RoundSetup{ opponent: Hand::Scissors, myself: Hand::Paper },
            RoundSetup{ opponent: Hand::Scissors, myself: Hand::Scissors },
        ];
        let expected_outcomes = vec![
            RoundOutcome{ winner: Outcome::Draw, score_opponent: 4, score_myself: 4},
            RoundOutcome{ winner: Outcome::MyselfWins, score_opponent: 1, score_myself: 8},
            RoundOutcome{ winner: Outcome::OpponentWins, score_opponent: 7, score_myself: 3},
            RoundOutcome{ winner: Outcome::OpponentWins, score_opponent: 8, score_myself: 1},
            RoundOutcome{ winner: Outcome::Draw, score_opponent: 5, score_myself: 5},
            RoundOutcome{ winner: Outcome::MyselfWins, score_opponent: 2, score_myself: 9},
            RoundOutcome{ winner: Outcome::MyselfWins, score_opponent: 3, score_myself: 7},
            RoundOutcome{ winner: Outcome::OpponentWins, score_opponent: 9, score_myself: 2},
            RoundOutcome{ winner: Outcome::Draw, score_opponent: 6, score_myself: 6},
        ];

        for (setup, expected) in round_setups.iter().zip(expected_outcomes.iter()) {
            // Act
            let round_outcome = setup.play();

            // Assert
            assert_eq!(round_outcome.winner, expected.winner, "Unexpected winner outcome.\nSetup: {:?}\nExpected: {:?}, Actual: {:?}\n", setup, expected.winner, round_outcome.winner);
            assert_eq!(round_outcome.score_opponent, expected.score_opponent, "Unexpected score_opponent.\nSetup: {:?}\nExpected: {:?}, Actual: {:?}\n", setup, expected.score_opponent, round_outcome.score_opponent);
            assert_eq!(round_outcome.score_myself, expected.score_myself, "Unexpected score_myself.\nSetup: {:?}\nExpected: {:?}, Actual: {:?}\n", setup, expected.score_myself, round_outcome.score_myself);
        }
        // Act
    }

    #[test]
    fn test_simple_line_parsing() {
        // Arrange
        let rounds = vec!["A X", "A Y", "A Z", "B X", "B Y", "B Z", "C X", "C Y", "C Z"];
        let expected_setups = vec![
            RoundSetup{ opponent: Hand::Rock, myself: Hand::Rock },
            RoundSetup{ opponent: Hand::Rock, myself: Hand::Paper },
            RoundSetup{ opponent: Hand::Rock, myself: Hand::Scissors },
            RoundSetup{ opponent: Hand::Paper, myself: Hand::Rock },
            RoundSetup{ opponent: Hand::Paper, myself: Hand::Paper },
            RoundSetup{ opponent: Hand::Paper, myself: Hand::Scissors },
            RoundSetup{ opponent: Hand::Scissors, myself: Hand::Rock },
            RoundSetup{ opponent: Hand::Scissors, myself: Hand::Paper },
            RoundSetup{ opponent: Hand::Scissors, myself: Hand::Scissors },
        ];

        for (round, expected) in rounds.iter().zip(expected_setups.iter()) {
            // Act
            let round_setup = RoundSetup::new(round, Problem::Part1);

            // Assert
            assert_eq!(round_setup.opponent, expected.opponent);
            assert_eq!(round_setup.myself, expected.myself);
        }
    }
}

