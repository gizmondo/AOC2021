use anyhow::{anyhow, Result};
use itertools::Itertools;
use std::collections::BTreeMap;
use std::convert::TryInto;
use std::io::{self, Read};

const SIZE: usize = 5;

struct Board<const N: usize> {
    positions: BTreeMap<i32, (usize, usize)>,
    marked_rows: [usize; N],
    marked_cols: [usize; N],
    won: bool,
}

impl<const N: usize> Board<N> {
    fn new(board: [[i32; N]; N]) -> Board<N> {
        let mut positions = BTreeMap::new();
        for (row, values) in board.iter().enumerate() {
            for (col, &value) in values.iter().enumerate() {
                positions.insert(value, (row, col));
            }
        }
        Board {
            positions: positions,
            marked_rows: [0; N],
            marked_cols: [0; N],
            won: false,
        }
    }

    fn mark(&mut self, number: i32) -> Option<i32> {
        if self.won {
            None
        } else {
            self.positions.remove(&number).and_then(|(row, col)| {
                self.marked_rows[row] += 1;
                self.marked_cols[col] += 1;
                if self.marked_rows[row] == N || self.marked_cols[col] == N {
                    self.won = true;
                    Some(self.positions.keys().sum::<i32>() * number)
                } else {
                    None
                }
            })
        }
    }
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let (moves_part, boards_part) = input
        .trim()
        .splitn(2, "\n\n")
        .collect_tuple()
        .ok_or(anyhow!("Can't find moves or board"))?;

    let mut boards: Vec<Board<SIZE>> = Vec::new();
    for board_raw in boards_part.split("\n\n") {
        let mut board: Vec<[i32; SIZE]> = Vec::new();
        for row in board_raw.split('\n') {
            let numbers = row
                .split_whitespace()
                .map(str::parse)
                .collect::<Result<Vec<i32>, _>>()?;
            board.push(numbers.try_into().unwrap());
        }
        boards.push(Board::new(board.try_into().unwrap()));
    }

    for move_raw in moves_part.split(',').map(str::parse) {
        let number = move_raw?;
        for (board_id, board) in boards.iter_mut().enumerate() {
            match board.mark(number) {
                Some(x) => println!("Board {} won: {}", board_id, x),
                None => (),
            }
        }
    }

    Ok(())
}
