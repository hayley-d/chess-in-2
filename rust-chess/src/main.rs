use colored::Color;
use colored::Colorize;
use rand::prelude::IndexedRandom;
use rand::seq::SliceRandom;
use std::fmt;

// Get an RNG:

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum PieceTypes {
    Null = 0,
    Pawn = 1,
    Knight = 3,
    Bishop = 4,
    Rook = 5,
    Queen = 9,
    King = 100,
}

impl PartialOrd for PieceTypes {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some((*self as i32).cmp(&(*other as i32)))
    }
}

impl Ord for PieceTypes {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as i32).cmp(&(*other as i32))
    }
}

#[derive(PartialEq, Copy, Eq)]
struct Piece {
    color: Color,
    value: PieceTypes,
    x: usize,
    y: usize,
}

impl PartialOrd for Piece {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl Ord for Piece {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}

struct Move {
    x: usize,
    y: usize,
}

impl Piece {
    pub fn new(color: Color, value: PieceTypes, x: usize, y: usize) -> Self {
        Piece { color, value, x, y }
    }
}

impl Clone for Piece {
    fn clone(&self) -> Self {
        *self
    }
}

#[warn(dead_code)]
struct GameBoard {
    board: [[Piece; 8]; 8],
}

impl GameBoard {
    fn new() -> Self {
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

    fn get_color(&self, color: Color) -> Vec<&Piece> {
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

impl std::default::Default for Piece {
    fn default() -> Self {
        Self {
            color: Color::Red,
            value: PieceTypes::Null,
            x: Default::default(),
            y: Default::default(),
        }
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

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

fn main() {
    let gameboard = GameBoard::new();

    game_loop(&gameboard);
}

fn is_black(col: usize, row: usize) -> bool {
    (col + row).is_multiple_of(2)
}

fn game_loop(gameboard: &GameBoard) {
    let is_white = determine_color();

    if is_white {
        println!("You are playing as white");
    } else {
        println!("You are playing as black");
    }

    println!("{}", gameboard);

    println!("Please select a piece to move from the following list:");
    let pieces = get_possible_pieces(is_white, gameboard);
}

fn determine_color() -> bool {
    let mut rng = rand::rng();
    let mut nums: Vec<i32> = (1..100).collect();
    nums.shuffle(&mut rng);
    let random_number = nums.choose(&mut rng).copied().unwrap_or(0);
    random_number % 2 == 0
}

fn get_possible_pieces(is_white: bool, gameboard: &GameBoard) -> Vec<&Piece> {
    let color = if is_white { Color::White } else { Color::Black };
    let pieces = gameboard.get_color(color);
    for piece in &pieces {
        let name = match piece.value {
            PieceTypes::Pawn => "Pawn",
            PieceTypes::Rook => "Rook",
            PieceTypes::Knight => "Knight",
            PieceTypes::Bishop => "Bishop",
            PieceTypes::Queen => "Queen",
            PieceTypes::King => "King",
            _ => " ",
        };

        let x = match piece.x {
            0 => "A",
            1 => "B",
            2 => "C",
            3 => "D",
            4 => "E",
            5 => "F",
            6 => "G",
            _ => "H",
        };

        println!("{} in {}{}", name, x, 8 - piece.y);
    }
    pieces
}

fn get_possible_moves(gameboard: &GameBoard) -> Vec<Move> {
    todo!()
}
