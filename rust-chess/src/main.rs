use colored::Colorize;
use std::fmt;

#[derive(PartialEq, Copy, Clone, Debug)]
enum PieceTypes {
    Null = 0,
    Pawn = 1,
    Knight = 3,
    Bishop = 4,
    Rook = 5,
    Queen = 9,
    King = 100,
}

#[derive(PartialEq, Copy)]
struct Piece {
    value: PieceTypes,
    x: usize,
    y: usize,
}

impl Piece {
    pub fn new(value: PieceTypes, x: usize, y: usize) -> Self {
        Piece { value, x, y }
    }
}

impl Clone for Piece {
    fn clone(&self) -> Self {
        *self
    }
}

#[warn(dead_code)]
struct GameBoard {
    board: [[Option<Box<Piece>>; 8]; 8],
}

impl GameBoard {
    fn new() -> Self {
        let mut board: [[Option<Box<Piece>>; 8]; 8] = Default::default();

        let mut x: usize = 0;
        while x < 8 {
            let mut y: usize = 0;
            while y < 8 {
                let p_type = match (x, y) {
                    (0, 0) | (7, 0) | (0, 7) | (7, 7) => PieceTypes::Rook,
                    (1, 0) | (1, 7) | (6, 0) | (6, 7) => PieceTypes::Knight,
                    (2, 0) | (2, 7) | (5, 0) | (5, 7) => PieceTypes::Bishop,
                    (3, 0) | (3, 7) => PieceTypes::Queen,
                    (4, 0) | (4, 7) => PieceTypes::King,
                    (_, 1) | (_, 6) => PieceTypes::Pawn,
                    _ => PieceTypes::Null,
                };

                if p_type != PieceTypes::Null {
                    board[x][y] = Some(Box::new(Piece::new(p_type, x, y)));
                }
                y += 1;
            }
            x += 1;
        }
        GameBoard { board }
    }
}

impl fmt::Display for PieceTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PieceTypes::Pawn => write!(f, " p "),
            PieceTypes::Knight => write!(f, "kn "),
            PieceTypes::Bishop => write!(f, " B "),
            PieceTypes::Rook => write!(f, " R "),
            PieceTypes::Queen => write!(f, " Q "),
            PieceTypes::King => write!(f, " K "),
            _ => write!(f, "   "),
        }
    }
}

impl fmt::Display for GameBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "    a   b   c   d   e   f   g   h")?;
        writeln!(f, "  ┌───┬───┬───┬───┬───┬───┬───┬───┐")?;

        for row in (0..8).rev() {
            write!(f, "{} │", row + 1)?;
            for col in 0..8 {
                let piece = &self.board[col][row];
                let symbol = match piece {
                    Some(p) => format!("{}", p),
                    None => "   ".to_string(),
                };
                if is_black(col, row) {
                    write!(f, "{}│", symbol.on_black().white())?;
                } else {
                    write!(f, "{}│", symbol.on_white().black())?;
                }
            }
            write!(f, " {}", row + 1)?;
            if row > 0 {
                writeln!(f)?;
                writeln!(f, "  ├───┼───┼───┼───┼───┼───┼───┼───┤")?;
            }
        }

        writeln!(f)?;
        writeln!(f, "  └───┴───┴───┴───┴───┴───┴───┴───┘")?;
        writeln!(f, "    a   b   c   d   e   f   g   h")?;
        Ok(())
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

fn main() {
    let gameboard = GameBoard::new();
    println!("{}", gameboard);
}

fn is_black(col: usize, row: usize) -> bool {
    (col + row) % 2 == 0
}
