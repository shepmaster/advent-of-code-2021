use snafu::{ensure, OptionExt, Snafu};

const INPUT: &str = include_str!("../input");

fn main() {
    println!("part1: {}", syntax_error_score(INPUT));
    println!("part2: {}", autocomplete_score(INPUT));
}

fn syntax_error_score(s: &str) -> u32 {
    syntax_error_score_inner(s).unwrap()
}

fn syntax_error_score_inner(s: &str) -> Result<u32> {
    let points = s.lines().map(|l| {
        match parse_line(l.trim()) {
            Ok(_) => Ok(0), // ignored
            Err(Error::Malformed { c }) => Ok(c.syntax_error_points().into()),
            Err(e) => Err(e),
        }
    });

    itertools::process_results(points, |p| p.sum())
}

fn autocomplete_score(s: &str) -> u64 {
    autocomplete_score_inner(s).unwrap()
}

fn autocomplete_score_inner(s: &str) -> Result<u64> {
    let points = s.lines().map(|l| {
        match parse_line(l.trim()) {
            Ok(s) => Ok(Some(s)),
            Err(Error::Malformed { .. }) => Ok(None), // ignored
            Err(e) => Err(e),
        }
    });

    itertools::process_results(points, |p| {
        let mut scores: Vec<_> = p
            .flatten()
            .map(|s| {
                s.into_iter()
                    .rev()
                    .fold(0, |acc, c| acc * 5 + u64::from(c.autocomplete_points()))
            })
            .collect();

        scores.sort_unstable();
        scores[scores.len() / 2]
    })
}

fn parse_line(s: &str) -> Result<Vec<Char>> {
    use OpenClose::*;

    let mut stack = Vec::new();
    for c in s.chars() {
        match OpenClose::from_char(c)? {
            Open(v) => stack.push(v),
            Close(v) => {
                let open = stack.pop().context(ExtraClosingSnafu)?;
                ensure!(open == v, MalformedSnafu { c: v });
            }
        }
    }
    Ok(stack)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum OpenClose {
    Open(Char),
    Close(Char),
}

impl OpenClose {
    fn from_char(c: char) -> Result<Self> {
        use {Char::*, OpenClose::*};

        Ok(match c {
            '{' => Open(CurlyBoi),
            '}' => Close(CurlyBoi),

            '(' => Open(RoundBoi),
            ')' => Close(RoundBoi),

            '[' => Open(SquareBoi),
            ']' => Close(SquareBoi),

            '<' => Open(PointyBoi),
            '>' => Close(PointyBoi),

            _ => return UnknownSnafu { c }.fail(),
        })
    }
}

#[allow(clippy::enum_variant_names)] // yeah boi
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Char {
    CurlyBoi,
    RoundBoi,
    SquareBoi,
    PointyBoi,
}

impl Char {
    fn syntax_error_points(&self) -> u16 {
        use Char::*;
        match self {
            RoundBoi => 3,
            SquareBoi => 57,
            CurlyBoi => 1197,
            PointyBoi => 25137,
        }
    }

    fn autocomplete_points(&self) -> u16 {
        use Char::*;
        match self {
            RoundBoi => 1,
            SquareBoi => 2,
            CurlyBoi => 3,
            PointyBoi => 4,
        }
    }
}

#[derive(Debug, Snafu)]
enum Error {
    ExtraClosing,
    Malformed { c: Char },
    Unknown { c: char },
}

type Result<T, E = Error> = std::result::Result<T, E>;

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test-input");

    #[test]
    fn test_part_1() {
        assert_eq!(26397, syntax_error_score(TEST_INPUT));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(288957, autocomplete_score(TEST_INPUT));
    }
}
