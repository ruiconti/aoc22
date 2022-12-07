#![feature(exclusive_range_pattern)]
#![feature(let_chains)]

use std::fs::read_to_string;

fn main() {
    // read from input.txt and call extract_calories_per_elf
    let input_contents = read_to_string("./input/calories.txt").expect("Unable to read input.txt");
    let mut elf_calories = extract_calories_per_elf(input_contents.as_str());

    elf_calories.sort_by(|a, b| b.cmp(a));
    let top_three = elf_calories.into_iter().take(3).collect::<Vec<_>>();
    println!("Top 3 most-economic elves: {:?}.\nTotal calories collected: {}", &top_three, top_three.iter().sum::<i32>());
}

fn extract_calories_per_elf(calories_notes: &str) -> Vec<i32> {
    let mut elf_calories = Vec::new();
    let mut elf_calorie = 0;

    for line in calories_notes.lines() {
        let sanitized_line = line.trim();
        if sanitized_line.is_empty() {
            elf_calories.push(elf_calorie);
            elf_calorie = 0;
        } else {
            elf_calorie += sanitized_line.parse::<i32>().expect(format!("Unable to parse line into i32 {}", line).as_str());
        }
    }

    if elf_calorie > 0 {
        elf_calories.push(elf_calorie);
    }
    elf_calories
}

#[cfg(test)]
mod test_extract_calories_per_elf {
    use super::*;

    #[test]
    fn without_ending_new_line() {
        let calories_notes = "100
    400
    300

    100
    50

    200";
        let elf_calories = extract_calories_per_elf(calories_notes);
        assert_eq!(elf_calories, vec![800, 150, 200]);
    }

    #[test]
    fn with_ending_new_line() {
        let calories_notes = "10
    900
    300
    55

    100
    200
    50

    300

    500

    200
    100
    30
    ";
        let elf_calories = extract_calories_per_elf(calories_notes);
        assert_eq!(elf_calories, vec![1265, 350, 300, 500, 330]);
    }
}


