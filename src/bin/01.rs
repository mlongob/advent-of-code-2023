use lazy_static::lazy_static;
use regex::{Captures, Regex};

advent_of_code::solution!(1);

fn convert_spelled_out_digits(line: &str) -> String {
    lazy_static! {
        static ref RE: Regex =
            Regex::new("(one|two|three|four|five|six|seven|eight|nine)").unwrap();
    }
    RE.replace_all(line, |cap: &Captures| match &cap[0] {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => panic!("Logic error"),
    })
    .to_string()
}

fn convert_spelled_out_digits_reversed(line: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            ")one|two|three|four|five|six|seven|eight|nine("
                .chars()
                .rev()
                .collect::<String>()
                .as_str()
        )
        .unwrap();
    }
    RE.replace_all(
        line.chars().rev().collect::<String>().as_str(),
        |cap: &Captures| match cap[0].chars().rev().collect::<String>().as_str() {
            "one" => "1",
            "two" => "2",
            "three" => "3",
            "four" => "4",
            "five" => "5",
            "six" => "6",
            "seven" => "7",
            "eight" => "8",
            "nine" => "9",
            _ => panic!("Logic error"),
        },
    )
    .to_string()
    .chars()
    .rev()
    .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter_map(|l| {
                let first = l.chars().find(char::is_ascii_digit)?.to_digit(10)?;
                let last = l.chars().rfind(char::is_ascii_digit)?.to_digit(10)?;
                Some(first * 10 + last)
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter_map(|l| {
                let first = convert_spelled_out_digits(l)
                    .chars()
                    .find(char::is_ascii_digit)?
                    .to_digit(10)?;
                let last = convert_spelled_out_digits_reversed(l)
                    .chars()
                    .rfind(char::is_ascii_digit)?
                    .to_digit(10)?;
                Some(first * 10 + last)
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_spelled_out_digits() {
        assert_eq!(convert_spelled_out_digits("one1one"), "111");
        assert_eq!(convert_spelled_out_digits("twone1one"), "2ne11");
    }

    #[test]
    fn test_convert_spelled_out_digits_reversed() {
        assert_eq!(convert_spelled_out_digits_reversed("one1one"), "111");
        assert_eq!(convert_spelled_out_digits_reversed("twone1one"), "tw111");
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
