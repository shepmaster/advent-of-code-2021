use std::{
    collections::BTreeMap,
    fmt::Display,
    ops::{Add, Sub},
    str::FromStr,
};

const INPUT: &str = include_str!("../input");

fn main() {
    println!("part1: {}", output_digits_with_unique_segments(INPUT));
    println!("part2: {}", output_value_sum(INPUT));
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

fn output_value_sum(s: &str) -> usize {
    output_value_sum_inner(s).unwrap()
}

fn output_value_sum_inner(s: &str) -> Result<usize> {
    let i = s.lines().map(|l| l.trim().parse::<Entry>());
    itertools::process_results(i, |i| i.map(|e| e.output_value()).sum())
}

#[derive(Debug, Copy, Clone)]
struct Entry(Input, Output);

impl Entry {
    fn output_digits_with_unique_segments(&self) -> usize {
        self.1.digits_with_unique_segments()
    }

    fn output_value(&self) -> usize {
        let analyzed = self.0.careful_analysis();

        self.1 .0.iter().fold(0, |sum, d| {
            let v = analyzed
                .iter()
                .enumerate()
                .find_map(|(i, a)| (a == d).then(|| i))
                .expect("No match found");
            sum * 10 + v
        })
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

impl Digits<10> {
    fn careful_analysis(&self) -> [Digit; 10] {
        let mut group = BTreeMap::<_, Vec<_>>::new();
        for &d in &self.0 {
            group.entry(d.n_segments()).or_default().push(d);
        }
        let fives = &group[&5];

        let s_1 = *group[&2].first().expect("Did not find 1");
        let s_4 = *group[&4].first().expect("Did not find 4");
        let s_7 = *group[&3].first().expect("Did not find 7");
        let s_8 = *group[&7].first().expect("Did not find 8");
        let s_3 = *fives
            .iter()
            .find(|c| c.contains(&s_7))
            .expect("Did not find 3");
        let s_9 = s_3 + s_4;
        let s_e = s_8 - s_9;
        let s_b = s_8 - s_3 - s_e;
        let s_2 = *fives
            .iter()
            .find(|c| c.contains(&s_e))
            .expect("Did not find 2");
        let s_5 = *fives
            .iter()
            .find(|c| c.contains(&s_b))
            .expect("Did not find 5");
        let s_6 = s_5 + s_e;
        let s_0 = *self
            .0
            .iter()
            .find(|d| ![s_1, s_2, s_3, s_4, s_5, s_6, s_7, s_8, s_9].contains(d))
            .expect("Dif not find 0");

        [s_0, s_1, s_2, s_3, s_4, s_5, s_6, s_7, s_8, s_9]
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

macro_rules! digit_literal {
    ($a:ident, $b:ident, $c:ident, $d:ident, $e:ident, $f:ident, $g:ident) => {
        Digit([
            digit_literal!(@ $a),
            digit_literal!(@ $b),
            digit_literal!(@ $c),
            digit_literal!(@ $d),
            digit_literal!(@ $e),
            digit_literal!(@ $f),
            digit_literal!(@ $g),
        ])
    };

    (@ t) => { true };
    (@ f) => { false };
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
struct Digit([bool; 7]);

impl Digit {
    //                              A  B  C  D  E  F  G
    const D0: Self = digit_literal!(t, t, t, f, t, t, t);
    const D1: Self = digit_literal!(f, f, t, f, f, t, f);
    const D2: Self = digit_literal!(t, f, t, t, t, f, t);
    const D3: Self = digit_literal!(t, f, t, t, f, t, t);
    const D4: Self = digit_literal!(f, t, t, t, f, t, f);
    const D5: Self = digit_literal!(t, t, f, t, f, t, t);
    const D6: Self = digit_literal!(t, t, f, t, t, t, t);
    const D7: Self = digit_literal!(t, f, t, f, f, t, f);
    const D8: Self = digit_literal!(t, t, t, t, t, t, t);
    const D9: Self = digit_literal!(t, t, t, t, f, t, t);

    #[allow(unused)]
    const DIGITS: [Self; 10] = [
        Self::D0,
        Self::D1,
        Self::D2,
        Self::D3,
        Self::D4,
        Self::D5,
        Self::D6,
        Self::D7,
        Self::D8,
        Self::D9,
    ];

    fn has_unique_segments(&self) -> bool {
        const SEGMENTS_FOR_1: usize = 2;
        const SEGMENTS_FOR_4: usize = 4;
        const SEGMENTS_FOR_7: usize = 3;
        const SEGMENTS_FOR_8: usize = 7;

        matches!(
            self.n_segments(),
            SEGMENTS_FOR_1 | SEGMENTS_FOR_4 | SEGMENTS_FOR_7 | SEGMENTS_FOR_8
        )
    }

    fn n_segments(&self) -> usize {
        self.0.iter().filter(|&&x| x).count()
    }

    fn contains(&self, other: &Digit) -> bool {
        other.active_segments().all(|i| self.0[i])
    }

    fn active_segments(&self) -> impl Iterator<Item = usize> + '_ {
        self.0.iter().enumerate().filter_map(|(i, s)| s.then(|| i))
    }
}

impl Add for Digit {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut this = self;

        for (t, &r) in this.0.iter_mut().zip(rhs.0.iter()) {
            if r {
                *t = true;
            }
        }

        this
    }
}

impl Sub for Digit {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut this = self;

        for (t, &r) in this.0.iter_mut().zip(rhs.0.iter()) {
            if r {
                *t = false;
            }
        }

        this
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

impl Display for Digit {
    fn fmt(&self, ft: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let a = if self.0[0] { 'a' } else { '.' };
        let b = if self.0[1] { 'b' } else { '.' };
        let c = if self.0[2] { 'c' } else { '.' };
        let d = if self.0[3] { 'd' } else { '.' };
        let e = if self.0[4] { 'e' } else { '.' };
        let f = if self.0[5] { 'f' } else { '.' };
        let g = if self.0[6] { 'g' } else { '.' };

        writeln!(ft, " {a}{a}{a}{a} ")?;
        writeln!(ft, "{b}    {c}")?;
        writeln!(ft, "{b}    {c}")?;
        writeln!(ft, " {d}{d}{d}{d} ")?;
        writeln!(ft, "{e}    {f}")?;
        writeln!(ft, "{e}    {f}")?;
        writeln!(ft, " {g}{g}{g}{g} ")?;

        Ok(())
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
    fn test_part_2() {
        assert_eq!(61229, output_value_sum(TEST_INPUT));
    }

    #[test]
    fn test_parsing() {
        "ecbad fdeacg gaecbd gbae gfcdbea cadge fcagdb abc cfdbe ab | beag bac dacgbe aegb"
            .parse::<Entry>()
            .unwrap();
    }

    #[test]
    fn test_careful_analysis() {
        let analyzed = Digits(Digit::DIGITS).careful_analysis();
        assert_eq!(analyzed, Digit::DIGITS);
    }
}
