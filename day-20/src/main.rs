use std::{
    collections::{BTreeMap, BTreeSet},
    mem,
    ops::RangeInclusive,
};

const INPUT: &str = include_str!("../input");

fn main() {
    // WRONG: 5096 (too high); algorithm[0] can be true, result in lighting up everything
    // WRONG: 5077 (too low); needed to treat 0b1_1111_1111 and 0b0_0000_0000 as special background tiles
    println!("part1: {}", lit_pixels_after_two_enhancements(INPUT));
}

fn lit_pixels_after_two_enhancements(s: &str) -> usize {
    let (algorithm, image) = parse_input(s);
    let image = apply_algorithm(&algorithm, image, 2);
    image.values().filter(|&&x| x).count()
}

type Algorithm = Box<[bool; ALGORITHM_LENGTH]>;
type Coord = (i64, i64);
type Image = BTreeMap<Coord, bool>;
#[allow(dead_code)]
type CoordSet = BTreeSet<Coord>;

const LIT: char = '#';
const ALGORITHM_LENGTH: usize = 512;
const ALGORITHM_MAX_IDX: usize = ALGORITHM_LENGTH - 1;

fn parse_input(s: &str) -> (Algorithm, Image) {
    let mut lines = s.lines();

    let algorithm = lines.by_ref().next().expect("Missing algorithm");
    let algorithm = algorithm
        .trim()
        .chars()
        .map(|c| c == LIT)
        .collect::<Vec<_>>()
        .into_boxed_slice()
        .try_into()
        .expect("Malformed algorithm");

    let image = (0..)
        .zip(lines.skip(1))
        .flat_map(|(y, l)| {
            (0..)
                .zip(l.trim().chars())
                .filter_map(move |(x, c)| (c == LIT).then(|| ((x, y), true)))
        })
        .collect();

    (algorithm, image)
}

// The algorithm sometimes transforms 0b0_0000_0000 to lit and
// 0b1_1111_1111 to unlit, which would flip every cell if we did an
// odd number of steps and require an infinite amount of
// storage. Instead, we detect when that would happen and remove
// those elements and ambiently recreate them on the next step.
fn apply_algorithm(algorithm: &Algorithm, mut image: Image, n_steps: usize) -> Image {
    assert!(
        algorithm[0] ^ algorithm[ALGORITHM_MAX_IDX],
        "Must toggle both 0b0_0000_000 and 0b1_1111_1111",
    );
    assert!(
        n_steps % 2 == 0,
        "Can only handle even steps due to inverse"
    );

    let inverts = algorithm[0];
    let mut is_inverted = false;

    let mut to_visit = BTreeSet::new();
    let mut next_image = Image::new();

    // let print_x = -10..=10;
    // let print_y = -10..=10;

    for _step in 0..n_steps {
        let relevant_coords = image.keys().flat_map(|&c| neighbors(c));
        to_visit.extend(relevant_coords);

        // println!("{_step}");
        // print_image(&image, &to_visit, is_inverted, print_x.clone(), print_y.clone());

        let next_coords = to_visit.iter().flat_map(|&c| {
            let index = neighbors(c).fold(0, |acc, i| {
                let lit = image.get(&i).copied().unwrap_or(is_inverted);
                acc << 1 | lit as usize
            });

            if !is_inverted && index == 0 || is_inverted && index == ALGORITHM_MAX_IDX {
                None
            } else {
                Some((c, algorithm[index]))
            }
        });
        next_image.extend(next_coords);

        mem::swap(&mut image, &mut next_image);
        next_image.clear();

        to_visit.clear();

        is_inverted ^= inverts;
    }

    // println!("final:");
    // print_image(&image, &Default::default(), is_inverted, print_x, print_y);

    assert!(!is_inverted, "Don't expect exiting as inverted");
    image
}

// Important to go top->down, left->right
fn neighbors((x0, y0): Coord) -> impl Iterator<Item = Coord> {
    (-1..=1).flat_map(move |y| (-1..=1).map(move |x| (x0 + x, y0 + y)))
}

#[allow(dead_code)]
fn print_image(
    image: &Image,
    to_visit: &CoordSet,
    is_inverted: bool,
    x: RangeInclusive<i64>,
    y: RangeInclusive<i64>,
) {
    let bg_on = ("128", "64", "64");
    let bg_off = ("0", "0", "0");

    for y in y {
        print!("{y:04} ");
        for x in x.clone() {
            let (r, g, b) = if to_visit.contains(&(x, y)) {
                bg_on
            } else {
                bg_off
            };
            let present = image.get(&(x, y)).copied().unwrap_or(is_inverted);
            let c = if present { LIT } else { ' ' };
            print!("\x1b[48;2;{r};{g};{b}m{c}\x1b[0m");
        }
        println!();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test-input");

    #[test]
    fn test_part1() {
        assert_eq!(35, lit_pixels_after_two_enhancements(TEST_INPUT));
    }
}
