use std::str::FromStr;

const INPUT: &str = include_str!("../input");

fn main() {
    println!("part1: {}", output_digits_with_unique_segments(INPUT));
}

fn output_digits_with_unique_segments(s: &str) -> usize {
    output_digits_with_unique_segments_inner(s).unwrap()
}

fn output_digits_with_unique_segments_inner(s: &str) -> Result<usize> {
    let i = s.lines().map(|l| l.trim().parse::<Entry>());
    itertools::process_results(i, |i| {
        i.map(|e| e.output_digits_with_unique_segments()).sum()
    })
}

#[derive(Debug, Copy, Clone)]
struct Entry(Input, Output);

impl Entry {
    fn output_digits_with_unique_segments(&self) -> usize {
        self.1.digits_with_unique_segments()
    }
}

impl FromStr for Entry {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (i, o) = s
            .trim()
            .split_once('|')
            .ok_or("Did not have exactly input and output")?;
        Ok(Self(i.trim().parse()?, o.trim().parse()?))
    }
}

type Input = Digits<10>;
type Output = Digits<4>;

#[derive(Debug, Copy, Clone)]
struct Digits<const N: usize>([Digit; N]);

impl<const N: usize> Digits<N> {
    fn digits_with_unique_segments(&self) -> usize {
        self.0.iter().filter(|d| d.has_unique_segments()).count()
    }
}

impl<const N: usize> FromStr for Digits<N> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut this = Self([Digit::default(); N]);
        for (i, word) in s.split_ascii_whitespace().enumerate() {
            let i = this.0.get_mut(i).ok_or("Too many digits")?;
            *i = word.trim().parse()?;
        }
        Ok(this)
    }
}

#[derive(Debug, Copy, Clone, Default)]
struct Digit([bool; 7]);

impl Digit {
    fn has_unique_segments(&self) -> bool {
        let n_segments = self.0.iter().filter(|&&x| x).count();

        const SEGMENTS_FOR_1: usize = 2;
        const SEGMENTS_FOR_4: usize = 4;
        const SEGMENTS_FOR_7: usize = 3;
        const SEGMENTS_FOR_8: usize = 7;
        matches!(
            n_segments,
            SEGMENTS_FOR_1 | SEGMENTS_FOR_4 | SEGMENTS_FOR_7 | SEGMENTS_FOR_8
        )
    }
}

impl FromStr for Digit {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut this = Self::default();
        for c in s.chars() {
            match c {
                'a' => this.0[0] = true,
                'b' => this.0[1] = true,
                'c' => this.0[2] = true,
                'd' => this.0[3] = true,
                'e' => this.0[4] = true,
                'f' => this.0[5] = true,
                'g' => this.0[6] = true,
                other => return Err(format!("Unknown character {:?}", other).into()),
            }
        }
        Ok(this)
    }
}

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test-input");

    #[test]
    fn test_part_1() {
        assert_eq!(26, output_digits_with_unique_segments(TEST_INPUT));
    }

    #[test]
    fn test_parsing() {
        "ecbad fdeacg gaecbd gbae gfcdbea cadge fcagdb abc cfdbe ab | beag bac dacgbe aegb"
            .parse::<Entry>()
            .unwrap();
    }
}
