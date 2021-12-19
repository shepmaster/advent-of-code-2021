use std::{iter::Sum, ops::Add};

const INPUT: &str = include_str!("../input");

fn main() {
    println!("part1: {}", magnitude_of_sum(INPUT));
}

fn magnitude_of_sum(s: &str) -> u32 {
    s.lines()
        .map(|l| Snailfish::parse(l.trim()))
        .sum::<Snailfish>()
        .magnitude()
}

#[derive(Debug, PartialEq, Eq)]
enum Snailfish {
    Literal(u32),
    Nested(Box<(Snailfish, Snailfish)>),
}

impl Snailfish {
    fn parse(mut s: &str) -> Self {
        fn parse_inner(s: &mut &str) -> Snailfish {
            match s.strip_prefix('[') {
                Some(mut inner) => {
                    let l = parse_inner(&mut inner);
                    inner = inner.trim_start_matches(',');
                    let r = parse_inner(&mut inner);
                    inner = inner.trim_start_matches(']');
                    *s = inner;
                    Snailfish::Nested(Box::new((l, r)))
                }
                None => {
                    let l = s
                        .chars()
                        .take_while(|c| c.is_ascii_digit())
                        .map(|c| c.len_utf8())
                        .sum();
                    let (d, r) = s.split_at(l);
                    let d = d.parse().expect("invalid digit");
                    *s = r;
                    Snailfish::Literal(d)
                }
            }
        }

        parse_inner(&mut s)
    }

    fn magnitude(&self) -> u32 {
        match self {
            Snailfish::Literal(v) => *v,
            Snailfish::Nested(c) => 3 * c.0.magnitude() + 2 * c.1.magnitude(),
        }
    }

    fn literal(&self) -> u32 {
        match self {
            Snailfish::Literal(v) => *v,
            Snailfish::Nested(_) => panic!("Value was not a literal"),
        }
    }

    fn add_unreduced(self, other: Self) -> Self {
        Snailfish::Nested(Box::new((self, other)))
    }

    fn reduce(&mut self) {
        loop {
            if self.explode() {
                continue;
            }
            if self.split() {
                continue;
            }
            break;
        }
    }

    fn explode(&mut self) -> bool {
        enum Exploded {
            No,
            Yes(Option<u32>, Option<u32>),
        }
        use {Exploded::*, Snailfish::*};

        fn explode_inner(this: &mut Snailfish, depth: usize) -> Exploded {
            match this {
                Literal(_) => No,
                Nested(c) => {
                    if depth >= 4 {
                        let l = c.0.literal();
                        let r = c.1.literal();
                        *this = Literal(0);
                        Yes(Some(l), Some(r))
                    } else if let Yes(l, r) = explode_inner(&mut c.0, depth + 1) {
                        match r {
                            Some(v) => {
                                if c.1.add_regular_left(v) {
                                    Yes(l, None)
                                } else {
                                    Yes(l, r)
                                }
                            }
                            None => Yes(l, r),
                        }
                    } else if let Yes(l, r) = explode_inner(&mut c.1, depth + 1) {
                        match l {
                            Some(v) => {
                                if c.0.add_regular_right(v) {
                                    Yes(None, r)
                                } else {
                                    Yes(l, r)
                                }
                            }
                            None => Yes(l, r),
                        }
                    } else {
                        No
                    }
                }
            }
        }

        matches!(explode_inner(self, 0), Exploded::Yes(..))
    }

    fn add_regular_left(&mut self, v: u32) -> bool {
        match self {
            Snailfish::Literal(c) => {
                *c += v;
                true
            }
            Snailfish::Nested(c) => c.0.add_regular_left(v) || c.1.add_regular_left(v),
        }
    }

    fn add_regular_right(&mut self, v: u32) -> bool {
        match self {
            Snailfish::Literal(c) => {
                *c += v;
                true
            }
            Snailfish::Nested(c) => c.1.add_regular_right(v) || c.0.add_regular_right(v),
        }
    }

    fn split(&mut self) -> bool {
        let v = match self {
            Snailfish::Literal(v) => *v,
            Snailfish::Nested(v) => return v.0.split() || v.1.split(),
        };

        if v < 10 {
            return false;
        }
        let v = f64::from(v) / 2.0;
        let l = v.floor() as u32;
        let r = v.ceil() as u32;

        *self = Self::Nested(Box::new((Self::Literal(l), Self::Literal(r))));
        true
    }
}

impl Add for Snailfish {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut v = Snailfish::add_unreduced(self, rhs);
        v.reduce();
        v
    }
}

impl Sum for Snailfish {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.reduce(Add::add).expect("Don't know addition identity")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test-input");

    #[test]
    fn test_part1() {
        let sum = TEST_INPUT
            .lines()
            .map(|l| Snailfish::parse(l.trim()))
            .sum::<Snailfish>();
        assert_eq!(
            sum,
            Snailfish::parse("[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]")
        );
        assert_eq!(4140, sum.magnitude());
    }

    #[test]
    fn parse() {
        Snailfish::parse("[1,2]");
        Snailfish::parse("[[1,2],3]");
        Snailfish::parse("[9,[8,7]]");
        Snailfish::parse("[[1,9],[8,5]]");
        Snailfish::parse("[[[[1,2],[3,4]],[[5,6],[7,8]]],9]");
        Snailfish::parse("[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]");
        Snailfish::parse("[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]");
    }

    #[test]
    fn add() {
        let a = Snailfish::parse("[[[[4,3],4],4],[7,[[8,4],9]]]") + Snailfish::parse("[1,1]");
        assert_eq!(Snailfish::parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"), a);
    }

    #[test]
    fn sum() {
        let a = ["[1,1]", "[2,2]", "[3,3]", "[4,4]"]
            .into_iter()
            .map(Snailfish::parse)
            .sum::<Snailfish>();
        assert_eq!(Snailfish::parse("[[[[1,1],[2,2]],[3,3]],[4,4]]"), a);

        let b = ["[1,1]", "[2,2]", "[3,3]", "[4,4]", "[5,5]"]
            .into_iter()
            .map(Snailfish::parse)
            .sum::<Snailfish>();
        assert_eq!(Snailfish::parse("[[[[3,0],[5,3]],[4,4]],[5,5]]"), b);

        let c = ["[1,1]", "[2,2]", "[3,3]", "[4,4]", "[5,5]", "[6,6]"]
            .into_iter()
            .map(Snailfish::parse)
            .sum::<Snailfish>();
        assert_eq!(Snailfish::parse("[[[[5,0],[7,4]],[5,5]],[6,6]]"), c);

        let d = [
            "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
            "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
            "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
            "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
            "[7,[5,[[3,8],[1,4]]]]",
            "[[2,[2,2]],[8,[8,1]]]",
            "[2,9]",
            "[1,[[[9,3],9],[[9,0],[0,7]]]]",
            "[[[5,[7,4]],7],1]",
            "[[[[4,2],2],6],[8,7]]",
        ]
        .into_iter()
        .map(Snailfish::parse)
        .sum::<Snailfish>();
        assert_eq!(
            Snailfish::parse("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"),
            d
        );
    }

    #[test]
    fn add_unreduced() {
        let a = Snailfish::parse("[1,2]");
        let b = Snailfish::parse("[[3,4],5]");
        let c = Snailfish::parse("[[1,2],[[3,4],5]]");

        assert_eq!(Snailfish::add_unreduced(a, b), c);
    }

    #[test]
    fn explode() {
        let mut a = Snailfish::parse("[[[[[9,8],1],2],3],4]");
        a.explode();
        assert_eq!(Snailfish::parse("[[[[0,9],2],3],4]"), a);

        let mut b = Snailfish::parse("[7,[6,[5,[4,[3,2]]]]]");
        b.explode();
        assert_eq!(Snailfish::parse("[7,[6,[5,[7,0]]]]"), b);

        let mut c = Snailfish::parse("[[6,[5,[4,[3,2]]]],1]");
        c.explode();
        assert_eq!(Snailfish::parse("[[6,[5,[7,0]]],3]"), c);

        let mut d = Snailfish::parse("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]");
        d.explode();
        assert_eq!(Snailfish::parse("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"), d);

        let mut e = Snailfish::parse("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
        e.explode();
        assert_eq!(Snailfish::parse("[[3,[2,[8,0]]],[9,[5,[7,0]]]]"), e);
    }

    #[test]
    fn split() {
        let mut a = Snailfish::parse("10");
        a.split();
        assert_eq!(Snailfish::parse("[5,5]"), a);

        let mut b = Snailfish::parse("11");
        b.split();
        assert_eq!(Snailfish::parse("[5,6]"), b);

        let mut c = Snailfish::parse("12");
        c.split();
        assert_eq!(Snailfish::parse("[6,6]"), c);
    }

    #[test]
    fn reduce() {
        let mut a = Snailfish::parse("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");
        a.reduce();
        assert_eq!(Snailfish::parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"), a);
    }

    #[test]
    fn magnitude() {
        assert_eq!(29, Snailfish::parse("[9,1]").magnitude());
        assert_eq!(21, Snailfish::parse("[1,9]").magnitude());
        assert_eq!(129, Snailfish::parse("[[9,1],[1,9]]").magnitude());

        assert_eq!(143, Snailfish::parse("[[1,2],[[3,4],5]]").magnitude());
        assert_eq!(
            1384,
            Snailfish::parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").magnitude()
        );
        assert_eq!(
            445,
            Snailfish::parse("[[[[1,1],[2,2]],[3,3]],[4,4]]").magnitude()
        );
        assert_eq!(
            791,
            Snailfish::parse("[[[[3,0],[5,3]],[4,4]],[5,5]]").magnitude()
        );
        assert_eq!(
            1137,
            Snailfish::parse("[[[[5,0],[7,4]],[5,5]],[6,6]]").magnitude()
        );
        assert_eq!(
            3488,
            Snailfish::parse("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]").magnitude()
        );
    }
}
