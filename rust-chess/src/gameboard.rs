use crate::piece::{Piece, PieceTypes};
use colored::{Color, Colorize};
use std::fmt;

#[warn(dead_code)]
pub struct GameBoard {
    pub board: [[Piece; 8]; 8],
}

impl Default for GameBoard {
    fn default() -> Self {
        Self::new()
    }
}

impl GameBoard {
    pub fn new() -> Self {
        let mut board: [[Piece; 8]; 8] = Default::default();

        let mut x: usize = 0;
        while x < 8 {
            let mut y: usize = 0;
            while y < 8 {
                let color = match (x, y) {
                    (_, 0) | (_, 1) => Color::Black,
                    (_, 7) | (_, 6) => Color::White,
                    _ => Color::Red,
                };

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
                    board[x][y] = Piece::new(color, p_type, x, y);
                }
                y += 1;
            }
            x += 1;
        }
        GameBoard { board }
    }

    pub fn get_color(&self, color: Color) -> Vec<&Piece> {
        let mut pieces: Vec<&Piece> = Vec::new();
        for row in 0..8 {
            for col in 0..8 {
                let piece = self.board[row][col];
                if piece.color == color && piece.value != PieceTypes::Null {
                    pieces.push(&self.board[row][col]);
                }
            }
        }

        pieces.sort_by(|a, b| (*a).cmp(*b));
        pieces
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
                let symbol = match piece.value {
                    PieceTypes::Null => "   ".to_string(),
                    p => format!("{}", p),
                };
                if is_black(col, row) {
                    write!(f, "{}│", symbol.color(Color::White).on_color(Color::Black))?;
                } else {
                    write!(f, "{}│", symbol.color(Color::Black).on_color(Color::White))?;
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

fn is_black(col: usize, row: usize) -> bool {
    (col + row).is_multiple_of(2)
}
