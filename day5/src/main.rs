use regex::Regex;
use std::fs::read_to_string;

fn main() {
    let input_path = "./input/crane-inst.txt";
    let input_contents =
        read_to_string(input_path).expect(format!("Unable to read {}", input_path).as_str());

    let game = Game::new(&input_contents);
    let crate_mover_models = vec![CrateMoverModel::Model9000, CrateMoverModel::Model9001];
    for mover_model in &crate_mover_models {
        println!(
            "Message using {:?}: {}\n",
            mover_model,
            game.find_message(*mover_model)
        );
    }
}

struct Game {
    stacks_raw: String,
    moves_raw: String,
}

impl Game {
    fn new(input: &str) -> Game {
        let (stacks_raw, moves_raw) = Game::parse_game_input(input);
        Game {
            stacks_raw: stacks_raw.to_string(),
            moves_raw: moves_raw.to_string(),
        }
    }

    fn extract_moves_from_game(moves_map: &str) -> Vec<Move> {
        let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        let mut moves = Vec::new();
        for move_raw in moves_map.lines() {
            let captures = re.captures(move_raw);
            match captures {
                Some(captures) => {
                    let amount = &captures[1].parse::<usize>().unwrap();
                    let from = &captures[2].parse::<usize>().unwrap();
                    let to = &captures[3].parse::<usize>().unwrap();
                    moves.push(Move {
                        amount: *amount,
                        from: *from,
                        to: *to,
                    });
                }
                None => {}
            }
        }
        moves
    }

    fn extract_stack_contents(stack_raw: &str) -> Vec<Vec<char>> {
        let (stack_indices, stack_ids_row) = extract_stack_indicies(stack_raw).unwrap();
        let stack_amount = stack_indices.len();
        let mut stacks = Vec::new();
        stacks.resize(stack_amount, Vec::new());

        for (row, line) in stack_raw.lines().enumerate() {
            if row == stack_ids_row {
                break;
            }
            let mut j = 0;
            for stack_index in 0..stack_amount {
                // We have, at most N stacks.
                // Each stack consumes 4 characters.
                // Therefore, to find out which stack we are in, we can divide the index by 4.
                let stack_item_raw = line.chars().skip(j).take(4).collect::<String>();
                if !stack_item_raw.trim().is_empty() {
                    let stack_item = extract_stack_item(stack_item_raw);
                    stacks[stack_index].push(stack_item);
                }
                j += 4;
            }
        }
        stacks
    }

    fn parse_game_input(input: &str) -> (String, String) {
        let cleaned_input = input
            .lines()
            .skip_while(|line| line.trim().is_empty()) // Skips any potential empty lines at the start
            .collect::<Vec<&str>>() // Collect it into a Vec<&str>
            .join("\n"); // join into a single string
        let re = Regex::new(r"\n{2,}").unwrap();

        let game = re
            .split(cleaned_input.as_str())
            .map(|line| line.to_string())
            .collect::<Vec<String>>();
        (game[0].clone(), game[1].clone())
    }

    fn execute_moves(&self, mover_model: CrateMoverModel) -> CrateMover {
        let stacks = Game::extract_stack_contents(self.stacks_raw.as_str());
        let moves = Game::extract_moves_from_game(self.moves_raw.as_str());
        let stacks_collection = CrateMover::new(stacks, mover_model);
        let new_stacks = stacks_collection.execute_moves(moves);
        CrateMover {
            stacks: new_stacks,
            model: mover_model,
        }
    }

    fn find_message(&self, mover_model: CrateMoverModel) -> String {
        let stack_collection = self.execute_moves(mover_model);
        let mut message: Vec<char> = Vec::new();
        for stack in stack_collection.stacks {
            message.push(stack[0].into());
        }
        let t = message.iter().collect::<String>();
        t
    }
}

#[cfg(test)]
mod test_full_game {
    use super::*;

    #[test]
    fn test_game_execute_moves_model9000() {
        let full_example = "
        [D]    
    [N] [C]    
    [Z] [M] [P]
    1   2   3 

    move 1 from 2 to 1
    move 3 from 1 to 3
    move 2 from 2 to 1
    move 1 from 1 to 2";

        let game = Game::new(full_example);

        let message = game.find_message(CrateMoverModel::Model9000);
        assert_eq!(message, "CMZ");
    }

    #[test]
    fn test_game_execute_moves_model9001() {
        let full_example = "
        [D]    
    [N] [C]    
    [Z] [M] [P]
    1   2   3 

    move 1 from 2 to 1
    move 3 from 1 to 3
    move 2 from 2 to 1
    move 1 from 1 to 2";

        let game = Game::new(full_example);

        let message = game.find_message(CrateMoverModel::Model9001);
        assert_eq!(message, "MCD");
    }

    #[test]
    fn test_parse_game_input() {
        let example = "
        
        

        [S] [A]
    [B] [C]
        [8] [C]

        move 1 from 4 to 2
        ";

        // act
        let (stacks_raw, moves_raw) = Game::parse_game_input(example);

        // assert
        assert_eq!(stacks_raw, "    [S] [A]\n[B] [C]\n    [8] [C]");
        assert_eq!(moves_raw, "    move 1 from 4 to 2\n    ");
    }
}

fn extract_stack_indicies(input: &str) -> Option<(Vec<&str>, usize)> {
    for (i, line) in input.lines().enumerate() {
        let stacks: Vec<&str> = line.matches(char::is_numeric).collect();
        if stacks.len() > 0 {
            return Some((stacks, i));
        }
        if line.is_empty() && i > 0 {
            // Skip potentially any initial empty lines
            break;
        }
    }
    None
}

fn extract_stack_item(stack_raw: String) -> char {
    let mut stack = stack_raw.chars();
    let stack_item = stack
        .nth(1)
        .expect(format!("unable to extract stack item: {:?}", stack_raw).as_str());
    stack_item
}

#[cfg(test)]
mod test_games_utils {
    use super::*;

    #[test]
    fn test_extract_stack_contents() {
        let example = "
        [D]    
    [N] [C]    
    [Z] [M] [P]
    [A] [B] [C]
    1   2   3";
        let stacks = Game::extract_stack_contents(example);
        assert_eq!(stacks[0], vec!['N', 'Z', 'A']);
        assert_eq!(stacks[1], vec!['D', 'C', 'M', 'B']);
        assert_eq!(stacks[2], vec!['P', 'C']);
    }
}
struct Move {
    amount: usize,
    from: usize,
    to: usize,
}

#[test]
fn test_extract_moves_from_game() {
    let example = "move 3 from 4 to 6
move 1 from 5 to 8
move 3 from 7 to 3
move 4 from 5 to 7
move 1 from 7 to 8";
    let want = vec![
        Move {
            amount: 3,
            from: 4,
            to: 6,
        },
        Move {
            amount: 1,
            from: 5,
            to: 8,
        },
        Move {
            amount: 3,
            from: 7,
            to: 3,
        },
        Move {
            amount: 4,
            from: 5,
            to: 7,
        },
        Move {
            amount: 1,
            from: 7,
            to: 8,
        },
    ];

    let moves = Game::extract_moves_from_game(example);
    for (m, w) in moves.into_iter().zip(want) {
        assert_eq!(m.amount, w.amount, "amounts did not match");
        assert_eq!(m.from, w.from, "from did not match");
        assert_eq!(m.to, w.to, "to did not match");
    }
}

#[derive(Debug, Copy, Clone)]
enum CrateMoverModel {
    Model9000,
    Model9001,
}

struct CrateMover {
    stacks: Vec<Vec<char>>,
    model: CrateMoverModel,
}

impl CrateMover {
    fn new(stacks: Vec<Vec<char>>, model: CrateMoverModel) -> CrateMover {
        CrateMover { stacks, model }
    }

    fn execute_moves(&self, moves: Vec<Move>) -> Vec<Vec<char>> {
        let mut stacks_new = self.stacks.clone();
        for m in moves {
            let mut items_to_move = stacks_new[m.from - 1]
                .drain(..m.amount)
                .collect::<Vec<char>>();
            println!(
                "items_to_move: {:?}; &self.stacks: {:?}",
                items_to_move,
                &self.stacks[m.from - 1]
            );
            match self.model {
                CrateMoverModel::Model9000 => {
                    for item in items_to_move {
                        stacks_new[m.to - 1].insert(0, item);
                    }
                }
                CrateMoverModel::Model9001 => {
                    items_to_move.reverse();
                    for item in items_to_move {
                        stacks_new[m.to - 1].insert(0, item);
                    }
                }
            }
        }
        stacks_new
    }
}

#[cfg(test)]
mod test_crate_mover {
    use super::*;

    #[test]
    fn test_execute_moves_simple_one_step() {
        let game = CrateMover::new(
            vec![vec!['N', 'Z'], vec!['D', 'C', 'M'], vec!['P']],
            CrateMoverModel::Model9000,
        );
        let moves = vec![Move {
            amount: 1,
            from: 2,
            to: 1,
        }];
        let expected = vec![vec!['D', 'N', 'Z'], vec!['C', 'M'], vec!['P']];

        let new_stack = game.execute_moves(moves);
        assert_eq!(new_stack, expected);
    }

    #[test]
    fn test_execute_moves_simple_two_steps() {
        let game = CrateMover::new(
            vec![vec!['N', 'Z'], vec!['D', 'C', 'M'], vec!['P']],
            CrateMoverModel::Model9000,
        );
        let moves = vec![
            Move {
                amount: 1,
                from: 2,
                to: 1,
            },
            Move {
                amount: 3,
                from: 1,
                to: 3,
            },
        ];
        let expected = vec![vec![], vec!['C', 'M'], vec!['Z', 'N', 'D', 'P']];

        let new_stack = game.execute_moves(moves);
        assert_eq!(new_stack, expected);
    }

    #[test]
    fn test_execute_moves_simple_three_steps() {
        let game = CrateMover::new(
            vec![vec!['N', 'Z'], vec!['D', 'C', 'M'], vec!['P']],
            CrateMoverModel::Model9000,
        );
        let moves = vec![
            Move {
                amount: 1,
                from: 2,
                to: 1,
            },
            Move {
                amount: 3,
                from: 1,
                to: 3,
            },
            Move {
                amount: 2,
                from: 2,
                to: 1,
            },
        ];
        let expected = vec![vec!['M', 'C'], vec![], vec!['Z', 'N', 'D', 'P']];

        let new_stack = game.execute_moves(moves);
        assert_eq!(new_stack, expected);
    }

    #[test]
    fn test_execute_moves_simple_four_steps() {
        let game = CrateMover::new(
            vec![vec!['N', 'Z'], vec!['D', 'C', 'M'], vec!['P']],
            CrateMoverModel::Model9000,
        );
        let moves = vec![
            Move {
                amount: 1,
                from: 2,
                to: 1,
            },
            Move {
                amount: 3,
                from: 1,
                to: 3,
            },
            Move {
                amount: 2,
                from: 2,
                to: 1,
            },
            Move {
                amount: 1,
                from: 1,
                to: 2,
            },
        ];
        let expected = vec![vec!['C'], vec!['M'], vec!['Z', 'N', 'D', 'P']];

        let new_stack = game.execute_moves(moves);
        assert_eq!(new_stack, expected);
    }
}
