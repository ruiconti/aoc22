use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
    let input_path = "./input/rucksacks.txt";
    let input_contents =
        read_to_string(input_path).expect(format!("Unable to read {}", input_path).as_str());
    let game1 = GamePart1::new(input_contents.as_str());
    let total_repetitive_items_priorities = game1.calculate_repetitive_item_priorities();
    println!(
        "Total repetitive items priorities: {}",
        total_repetitive_items_priorities
    );

    let game2 = GamePart2::new(game1.rucksacks);
    let total_badges_priorities = game2.calculate_group_badges_priorities();
    println!("Total badges priorities: {}", total_badges_priorities);
}

// Metadata:
// Each line represents a ruckstack.
// Each line has a string of chars, where each char represents an item.
// Each item is represented by a char. Each char is unique i.e. it is case-sensitive.
// Each item has a priority value, which is the index of the char in the alphabet.

// Rules:
// 1. Compartments
// A given ruckstack has two compartments. It has the same number of items in each of its two compartments.
// The first half of the characters represent items in the first compartment, while the second half of the characters represent items in the second compartment.
// All items of a given type are meant to go into exactly one of the two compartments.
//
// 2. Repetitive item type
// The Elf that did the packing failed to follow this rule for exactly one item type per rucksack.
// There can be an arbitrary number of items of this type in either compartment.

#[derive(Clone, Debug)]
struct Rucksack {
    items: Vec<char>,
    compartments: (HashSet<char>, HashSet<char>),
    repetitive_item: char,
}

impl Rucksack {
    fn new(line: &str) -> Self {
        // let mut compartments = (Vec::new(), Vec::new());
        let sanitized_line = line.trim();
        if sanitized_line.is_empty() {
            panic!("Invalid odd line: {}", line);
        }

        // Invariant: Only even lines as the size of compartments should be the same.
        if sanitized_line.len() % 2 != 0 {
            panic!("Invalid odd line: {}", line);
        }

        let (first_compartment, second_compartment) =
            sanitized_line.split_at(sanitized_line.len() / 2);
        let first_compartment_set: HashSet<char> = first_compartment.chars().collect();
        let second_compartment_set: HashSet<char> = second_compartment.chars().collect();
        let mut repetitive_items = first_compartment_set.intersection(&second_compartment_set);

        // Invariant: One repetitive item type.
        if repetitive_items.clone().count() != 1 {
            panic!("Invalid repetitive items: {}", line);
        }

        let repetitive_item = repetitive_items
            .next()
            .expect("Unexpected unwrapping of repetitive_items");

        Rucksack {
            compartments: (
                first_compartment_set.to_owned(),
                second_compartment_set.to_owned(),
            ),
            items: sanitized_line.chars().collect(),
            repetitive_item: repetitive_item.clone(),
        }
    }

    fn get_items(&self) -> &Vec<char> {
        &self.items
    }

    fn get_item_priority(item_type: Option<char>, rucksack: Option<&Rucksack>) -> i32 {
        let item = match item_type {
            Some(item_type) => item_type,
            None => match rucksack {
                Some(rucksack) => rucksack.repetitive_item,
                None => panic!("Invalid param usage"),
            },
        };

        let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let index = alphabet
            .find(item)
            .expect(format!("Unable to find item type {} in alphabet", item).as_str())
            as i32;
        return index + 1;
    }
}

#[cfg(test)]
mod test_rucksack {
    use super::*;

    #[test]
    fn test_create_rucksack_basic() {
        // Arrange & act
        let rucksack = Rucksack::new("ffabcCBADf");

        // Assert
        assert_eq!(rucksack.compartments.0.len(), 4); // Note that we're comparing the size of sets, so if the repetitive item repeats, the sizes will be different.
        assert_eq!(rucksack.compartments.1.len(), 5);
        assert_eq!(
            rucksack.compartments.0,
            vec!['f', 'a', 'b', 'c']
                .into_iter()
                .collect::<HashSet<char>>()
        );
        assert_eq!(
            rucksack.compartments.1,
            vec!['C', 'B', 'A', 'D', 'f']
                .into_iter()
                .collect::<HashSet<char>>()
        );
        assert_eq!(rucksack.repetitive_item, 'f');
    }

    #[test]
    fn test_create_rucksack_examples() {
        // Arrange
        let rucksacks = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
        ];
        let expected_repetitive = vec![
            'p', // 16
            'L', // 38
            'P', // 42
            'v', // 22
            't', // 20
            's', // 19
        ];
        let mut total = 0;
        let expected_total = 157;

        // Act
        for (rucksack, expected_repetitive_item) in rucksacks.iter().zip(expected_repetitive.iter())
        {
            let rucksack = Rucksack::new(rucksack);
            // Assert iterative
            assert_eq!(rucksack.repetitive_item, *expected_repetitive_item);
            total += Rucksack::get_item_priority(Some(*expected_repetitive_item), None);
        }
        // Assert total
        assert_eq!(total, expected_total);
    }

    #[test]
    fn test_map_item_type_to_priority() {
        assert_eq!(Rucksack::get_item_priority(Some('a'), None), 1);
        assert_eq!(Rucksack::get_item_priority(Some('z'), None), 26);
        assert_eq!(Rucksack::get_item_priority(Some('A'), None), 27);
        assert_eq!(Rucksack::get_item_priority(Some('Z'), None), 52);
    }
}

struct GamePart1 {
    rucksacks: Vec<Rucksack>,
}

trait RucksackGamePart1 {
    fn new(input_contents: &str) -> Self;
    fn calculate_repetitive_item_priorities(&self) -> i32;
}

impl RucksackGamePart1 for GamePart1 {
    fn new(input_contents: &str) -> Self {
        let mut rucksacks = Vec::<Rucksack>::new();
        for line in input_contents.lines() {
            if line.trim().is_empty() {
                continue;
            }
            let rucksack = Rucksack::new(line);
            rucksacks.push(rucksack);
        }
        GamePart1 { rucksacks }
    }

    fn calculate_repetitive_item_priorities(&self) -> i32 {
        return self.rucksacks.iter().fold(0, |acc, rucksack| {
            acc + Rucksack::get_item_priority(None, Some(&rucksack))
        });
    }
}

#[cfg(test)]
mod test_game_part1 {
    use super::*;

    #[test]
    fn test_rucksack_game_examples() {
        // Arrange
        let rucksacks = "
    vJrwpWtwJgWrhcsFMMfFFhFp
    jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL

    PmmdzqPrVvPwwTWBwg
    wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
    ttgJtRGJQctTZtZT
    CrZsJsPPZsGzwwsLwLmpwMDw";
        let expected_total = 157;

        // Act
        let game = GamePart1::new(rucksacks);

        // Assert
        assert_eq!(game.calculate_repetitive_item_priorities(), expected_total);
    }
}

#[derive(Debug)]
struct ElfGroup {
    elves: Vec<Rucksack>,
}

impl ElfGroup {
    fn find_badge(&self) -> char {
        let items: Vec<HashSet<char>> = self
            .elves
            .iter()
            .map(|rucksack| rucksack.get_items().to_owned().into_iter().collect())
            .collect();

        // Too much trouble to fold this one :(
        let mut badges = items[0].to_owned();
        for item in items.iter().skip(1) {
            badges = item.intersection(&badges).cloned().collect();
        }

        if badges.len() != 1 {
            panic!("Incorrect badges length: {}", badges.len());
        }
        badges.into_iter().next().expect("Invalid badge iter")
    }
}

#[cfg(test)]
mod test_elf_group {
    use super::*;

    #[test]
    fn test_find_badge_example() {
        // Arrange
        let group = ElfGroup {
            elves: vec![
                Rucksack::new("vJrwpWtwJgWrhcsFMMfFFhFp"),
                Rucksack::new("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
                Rucksack::new("PmmdzqPrVvPwwTWBwg"),
            ],
        };

        // Act & assert
        assert_eq!(group.find_badge(), 'r');
    }
}

struct GamePart2 {
    elf_groups: Vec<ElfGroup>,
}

trait RucksackGamePart2 {
    const GROUP_SIZE: usize;
    fn new(rucksacks: Vec<Rucksack>) -> Self;
    fn calculate_group_badges_priorities(&self) -> i32;
}

impl RucksackGamePart2 for GamePart2 {
    const GROUP_SIZE: usize = 3;

    fn new(rucksacks: Vec<Rucksack>) -> Self {
        let mut elf_groups = Vec::<ElfGroup>::new();

        let mut i = 0;
        while i < rucksacks.len() {
            let mut group = Vec::new();
            let mut j = i;
            while j < i + GamePart2::GROUP_SIZE {
                group.push(rucksacks[j].to_owned());
                j += 1;
            }
            elf_groups.push(ElfGroup { elves: group });
            i += GamePart2::GROUP_SIZE;
        }
        GamePart2 { elf_groups }
    }

    fn calculate_group_badges_priorities(&self) -> i32 {
        self.elf_groups.iter().fold(0, |acc, elf_group| {
            acc + Rucksack::get_item_priority(Some(elf_group.find_badge()), None)
        })
    }
}

#[cfg(test)]
mod test_game_part2 {
    use super::*;

    #[test]
    fn test_elf_group_example() {
        let rucksacks = "
    vJrwpWtwJgWrhcsFMMfFFhFp
    jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
    PmmdzqPrVvPwwTWBwg
    wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
    ttgJtRGJQctTZtZT
    CrZsJsPPZsGzwwsLwLmpwMDw
    ";
        let game1 = GamePart1::new(rucksacks);
        let game2 = GamePart2::new(game1.rucksacks);

        let total_groups_badges = game2.calculate_group_badges_priorities();
        assert_eq!(total_groups_badges, 70);
    }
}
