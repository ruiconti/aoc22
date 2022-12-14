use std::fs::read_to_string;

fn main() {
    let input_path = "./input/assignment_pairs.txt";
    let input_contents =
        read_to_string(input_path).expect(format!("Unable to read {}", input_path).as_str());

    let game = Game::new(input_contents.as_str());
    println!(
        "Number of fully contained pairs: {}\nNumber of overlapping pairs: {}",
        game.count_fully_contained_pairs(),
        game.count_overlapping_pairs()
    );
}

// Each elf is in charge for cleaning a range of sections. Each session has a unique ID number.
// However, an elf's assignment might overlap with another elf's assignment.
// Therefore, it is up to us to identify, in a pair of assignments, which ones are overlapping.

// For example, consider the following list of section assignment pairs:
//
//    2-4,6-8
//    2-3,4-5
//    5-7,7-9
//    2-8,3-7
//    6-6,4-6
//    2-6,4-8
//
// For the first few pairs, this list means:
//
// -    Within the first pair of Elves, the first Elf was assigned sections 2-4 (sections 2, 3, and 4),
//      while the second Elf was assigned sections 6-8 (sections 6, 7, 8).
// -    The Elves in the second pair were each assigned two sections.
// -    The Elves in the third pair were each assigned three sections: one got sections 5, 6, and 7, while
//      the other also got 7, plus 8 and 9.
//
// In how many assignment pairs does one range fully contain the other?
type Assignment = (i32, i32);

#[derive(Debug, Clone, Copy)]
struct AssignmentPair(Assignment, Assignment);

impl AssignmentPair {
    fn new(a: Assignment, b: Assignment) -> Self {
        AssignmentPair(a, b)
    }

    fn overlap(a: &Assignment, b: &Assignment) -> bool {
        /* does `a` overlaps `b`?
        Three possible cases
        case-1:
            a: ..3456...  a.0=3; a.1=6
            b: ....5678.  b.0=5; b.1=8
            Rationale: reference point: b.0; a must be between b.0
        case-2:
            a: ..3456...  a.0=3; a.1=6
            b: .234567..  b.0=2; b.1=7
            Rationale: reference point: b.0 and b.1; a must be within b.0 and b.1
        case-3:
            a: ..3456...  a.0=3; a.1=6
            b: .234.....  b.0=2; b.1=4
            Rationale: reference point: b.1; a must be between b.1
        */
        a.0 <= b.0 && a.1 >= b.0 /* case-1 */ ||
        a.0 >= b.0 && a.1 <= b.1 /* case-2 */ ||
        a.0 <= b.1 && a.1 >= b.1 /* case-3 */
    }

    fn contain(a: &Assignment, b: &Assignment) -> bool {
        /* does `a` contains `b`? */
        a.0 <= b.0 /* its lowest bound must be eq-lower */ && a.1 >= b.1 /* its upper bound must be eq-higher */
    }

    fn either_contains(&self) -> bool {
        /* does `self` contains `other` */
        AssignmentPair::contain(&self.0, &self.1) || AssignmentPair::contain(&self.1, &self.0)
    }

    fn either_overlaps(&self) -> bool {
        AssignmentPair::overlap(&self.0, &self.1) || AssignmentPair::overlap(&self.1, &self.0)
    }
}

impl Into<bool> for AssignmentPair {
    fn into(self) -> bool {
        self.0 .0 >= 0 && self.0 .1 >= 0 && self.1 .0 >= 0 && self.1 .1 >= 0
    }
}

impl PartialEq for AssignmentPair {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

#[cfg(test)]
mod test_assignment_pairs {
    use super::*;

    #[test]
    fn test_either_contains() {
        let examples = vec![
            ((2, 4), (6, 8), false),
            ((2, 3), (4, 5), false),
            ((5, 7), (7, 9), false),
            ((2, 8), (3, 7), true),
            ((6, 6), (4, 6), true),
            ((2, 6), (4, 8), false),
        ];

        for (a, b, expected) in examples {
            let pair = AssignmentPair::new(a, b);
            assert_eq!(pair.either_contains(), expected);
        }
    }

    #[test]
    fn test_either_overlaps() {
        let examples = vec![
            ((2, 4), (6, 8), false),
            ((2, 3), (4, 5), false),
            ((5, 7), (7, 9), true),
            ((2, 8), (3, 7), true),
            ((6, 6), (4, 6), true),
            ((2, 6), (4, 8), true),
        ];

        for (a, b, expected) in examples {
            let pair = AssignmentPair::new(a, b);
            assert_eq!(
                pair.either_overlaps(),
                expected,
                "did not overlap: {:?} {:?}",
                a,
                b
            );
        }
    }

    #[test]
    fn test_overloaded_truthyness_and_eq_operations() {
        assert_eq!(
            <AssignmentPair as Into<bool>>::into(AssignmentPair::new((2, 4), (6, 8))),
            true
        );
        assert_eq!(
            <AssignmentPair as Into<bool>>::into(AssignmentPair::new((0, 0), (0, 0))),
            true
        );
        assert_eq!(
            <AssignmentPair as Into<bool>>::into(AssignmentPair::new((0, 0), (0, -1))),
            false
        );
        assert_eq!(
            <AssignmentPair as Into<bool>>::into(AssignmentPair::new((0, 0), (-1, 0))),
            false
        );
        assert_eq!(
            <AssignmentPair as Into<bool>>::into(AssignmentPair::new((-1, 0), (0, 0))),
            false
        );
        assert_eq!(
            <AssignmentPair as Into<bool>>::into(AssignmentPair::new((0, -1), (0, 0))),
            false
        );
    }
}

struct Game {
    pairs: Vec<AssignmentPair>,
}

impl Game {
    fn new(raw_pairs: &str) -> Self {
        let pairs = raw_pairs
            .lines()
            .map(|line| {
                let cleaned_line = line.trim();
                if cleaned_line.is_empty() {
                    // That's a zero pair.
                    return AssignmentPair((-1, -1), (-1, -1));
                }
                let mut parts = line.trim().split(',');
                let first_assignment_raw = parts
                    .next()
                    .expect(format!("Missing first assignment, raw line: {}", line).as_str());
                let second_assignment_raw = parts
                    .next()
                    .expect(format!("Missing second assignment, raw line: {}", line).as_str());
                AssignmentPair(
                    Game::extract_assignment_raw(first_assignment_raw),
                    Game::extract_assignment_raw(second_assignment_raw),
                )
            })
            // Filter empty line
            .filter(|pair| <AssignmentPair as Into<bool>>::into(*pair))
            .collect();
        Game { pairs }
    }

    fn extract_assignment_raw(assignment_raw: &str) -> Assignment {
        let mut parts = assignment_raw.split('-');
        let lower = parts
            .next()
            .expect("Missing lower bound")
            .parse::<i32>()
            .expect("Unable to parse lower bound into i32");
        let upper = parts
            .next()
            .expect("Missing upper bound")
            .parse::<i32>()
            .expect("Unable to parse upper bound into i32");
        (lower, upper)
    }

    fn count_fully_contained_pairs(&self) -> usize {
        let mut count = 0;
        for pair in self.pairs.to_owned() {
            if pair.either_contains() {
                count += 1;
            }
        }
        count
    }

    fn count_overlapping_pairs(&self) -> usize {
        let mut count = 0;
        for pair in self.pairs.to_owned() {
            if pair.either_overlaps() {
                count += 1;
            }
        }
        count
    }
}

#[cfg(test)]
mod test_game {
    use super::*;

    #[test]
    fn test_input_pair_parsing() {
        let examples = "
    2-4,6-8
    2-3,4-5
    5-7,7-9
    2-8,3-7
    6-6,4-6
    ";
        let wanted = vec![
            AssignmentPair::new((2, 4), (6, 8)),
            AssignmentPair::new((2, 3), (4, 5)),
            AssignmentPair::new((5, 7), (7, 9)),
            AssignmentPair::new((2, 8), (3, 7)),
            AssignmentPair::new((6, 6), (4, 6)),
        ];
        let game = Game::new(examples);
        for (pair_got, pair_wanted) in game.pairs.iter().zip(wanted.iter()) {
            assert_eq!(pair_got.0, pair_wanted.0);
            assert_eq!(pair_got.1, pair_wanted.1);
        }
    }

    #[test]
    fn test_count_fully_contained_pairs_example() {
        let examples = "
    2-4,6-8
    2-3,4-5
    5-7,7-9
    2-8,3-7
    6-6,4-6
    2-6,4-8";

        let game = Game::new(examples);
        assert_eq!(game.count_fully_contained_pairs(), 2);
    }

    #[test]
    fn test_count_overlaps_example() {
        let examples = "
    2-4,6-8
    2-3,4-5
    5-7,7-9
    2-8,3-7
    6-6,4-6
    2-6,4-8";

        let game = Game::new(examples);
        assert_eq!(game.count_overlapping_pairs(), 4);
    }
}
