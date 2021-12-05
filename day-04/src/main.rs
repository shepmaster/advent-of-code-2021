#![feature(vec_retain_mut)]

use itertools::Itertools;
use std::collections::BTreeMap;

const INPUT: &str = include_str!("../input");

fn main() {
    // Wrong: 0 -- there are more than 3 boards
    println!("part1: {}", first_winning_score(INPUT));
    println!("part2: {}", last_winning_score(INPUT));
}

fn first_winning_score(s: &str) -> u64 {
    let (calls, mut boards) = parse_input(s);

    for call in calls {
        for board in &mut boards {
            board_mark_call(board, call);

            if board_is_win(board) {
                print_board(board);
                return board_sum_of_unmarked(board) * u64::from(call);
            }
        }
    }

    panic!("no winners");
}

fn last_winning_score(s: &str) -> u64 {
    let (mut calls, mut boards) = parse_input(s);

    for call in &mut calls {
        boards.retain_mut(|board| {
            board_mark_call(board, call);
            !board_is_win(board)
        });

        if boards.len() == 1 {
            break;
        }
    }

    let mut last = boards.pop().expect("Must have one board left");

    for call in calls {
        board_mark_call(&mut last, call);

        if board_is_win(&last) {
            print_board(&last);
            return board_sum_of_unmarked(&last) * u64::from(call);
        }
    }

    panic!("no winners");
}

fn parse_input(s: &str) -> (impl Iterator<Item = u8> + '_, Vec<Board>) {
    let mut lines = s.lines().peekable();
    let calls = lines.next().expect("Missing calls");
    let calls = calls.split(',').flat_map(str::parse);

    let mut boards = vec![];
    while lines.peek().is_some() {
        boards.push(parse_board(&mut lines));
    }

    (calls, boards)
}

type Board = BTreeMap<(usize, usize), (u8, bool)>;
const BOARD_DIMENSION: usize = 5;

fn parse_board<'a>(lines: impl Iterator<Item = &'a str>) -> Board {
    let mut board = BTreeMap::new();
    for (y, l) in lines.dropping(1).take(BOARD_DIMENSION).enumerate() {
        for (x, c) in l.split_ascii_whitespace().enumerate() {
            let c = c.parse().expect("Invalid digit");
            board.insert((x, y), (c, false));
        }
    }
    board
}

fn print_board(this: &Board) {
    for y in 0..BOARD_DIMENSION {
        for x in 0..BOARD_DIMENSION {
            let &(v, marked) = this.get(&(x, y)).unwrap();
            if marked {
                print!("\x1b[1m{:02}\x1b[0m ", v);
            } else {
                print!("{:02} ", v);
            }
        }
        println!();
    }
}

fn board_mark_call(this: &mut Board, call: u8) {
    for (num, seen) in this.values_mut() {
        if *num == call {
            *seen = true;
        }
    }
}

fn board_is_win(this: &Board) -> bool {
    board_is_win_vertical(this) || board_is_win_horizontal(this)
}

fn board_is_win_vertical(this: &Board) -> bool {
    (0..BOARD_DIMENSION)
        .any(|x| (0..BOARD_DIMENSION).all(|y| this.get(&(x, y)).map_or(false, |(_, v)| *v)))
}
fn board_is_win_horizontal(this: &Board) -> bool {
    (0..BOARD_DIMENSION)
        .any(|y| (0..BOARD_DIMENSION).all(|x| this.get(&(x, y)).map_or(false, |(_, v)| *v)))
}

fn board_sum_of_unmarked(this: &Board) -> u64 {
    this.values()
        .filter_map(|(v, marked)| (!marked).then(|| u64::from(*v)))
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test-input");

    #[test]
    fn test_part_1() {
        assert_eq!(4512, first_winning_score(TEST_INPUT));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(1924, last_winning_score(TEST_INPUT));
    }
}
