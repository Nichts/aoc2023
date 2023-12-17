use itertools::enumerate;
use nom::branch::alt;
use nom::error::{ErrorKind, ParseError};
use nom::IResult;
use std::str;

advent_of_code::solution!(1);

fn get_char_digit(c: Option<char>) -> u32 {
    c.unwrap().to_digit(10).unwrap()
}

const WORDS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn digit_word(reverse: bool) -> impl FnMut(&[u8]) -> IResult<&[u8], u8> {
    move |input: &[u8]| -> IResult<&[u8], u8> {
        let len = input.len();
        'outer: for (index, word) in enumerate(WORDS) {
            if word.len() > len {
                continue;
            }
            if reverse {
                for (a, b) in input.iter().rev().zip(word.as_bytes().iter().rev()) {
                    if a != b {
                        continue 'outer;
                    }
                }
            } else {
                for (a, b) in input.iter().zip(word.as_bytes().iter()) {
                    if a != b {
                        continue 'outer;
                    }
                }
            };
            return Ok((&input[..word.len()], index as u8 + 1));
        }
        let e: ErrorKind = ErrorKind::Tag;
        Err(nom::Err::Error(nom::error::Error::from_error_kind(
            input, e,
        )))
    }
}

fn digit_at(end: bool) -> impl FnMut(&[u8]) -> IResult<&[u8], u8> {
    move |input: &[u8]| -> IResult<&[u8], u8> {
        let index = if end { input.len() - 1 } else { 0 };
        if input[index].is_ascii_digit() {
            let remainder = if end { &input[..index] } else { &input[1..] };
            return Ok((remainder, input[index] - b'0'));
        }
        let e: ErrorKind = ErrorKind::Digit;
        Err(nom::Err::Error(nom::error::Error::from_error_kind(
            input, e,
        )))
    }
}

fn find_digit(input: &[u8], reverse: bool) -> u8 {
    let len = input.len();
    let mut parser = alt((digit_at(reverse), digit_word(reverse)));
    for i in 0..len {
        let slice = if reverse {
            &input[..(len - i)]
        } else {
            &input[i..]
        };
        let res = parser(slice);
        match res {
            Ok((_, res)) => return res,
            Err(nom::Err::Failure(error)) => {
                panic!("parsing failed {error:?}")
            }
            _ => (),
        }
    }
    let s = str::from_utf8(input).unwrap();
    panic!("no digit found in {s}")
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                get_char_digit(line.chars().find(|c| c.is_numeric())) * 10
                    + get_char_digit(line.chars().rfind(|c| c.is_numeric()))
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let input = line.as_bytes();
                let first = find_digit(input, false) as u32;
                let last = find_digit(input, true) as u32;
                first * 10 + last
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
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
