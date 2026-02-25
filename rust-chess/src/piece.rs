use crate::gameboard::*;
use colored::Color;
use std::fmt;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum PieceTypes {
    Null = 0,
    Pawn = 1,
    Knight = 3,
    Bishop = 4,
    Rook = 5,
    Queen = 9,
    King = 100,
}

#[derive(PartialEq, Copy, Eq)]
pub struct Piece {
    pub color: Color,
    pub value: PieceTypes,
    pub x: usize,
    pub y: usize,
}

#[derive(Clone)]
pub struct PlayerMove {
    pub x: usize,
    pub y: usize,
}

impl PlayerMove {
    pub fn new(x: usize, y: usize) -> Self {
        PlayerMove { x, y }
    }
}

pub trait Move {
    fn move_piece(&self, player_move: &PlayerMove) -> bool;
}

impl Move for Piece {
    fn move_piece(&self, player_move: &PlayerMove) -> bool {
        todo!()
    }
}

impl PartialOrd for PieceTypes {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PieceTypes {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as i32).cmp(&(*other as i32))
    }
}

impl PartialOrd for Piece {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Piece {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
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

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Piece {
    pub fn valid_move(&self, destination: &PlayerMove, gameboard: &GameBoard) -> bool {
        if gameboard.board[destination.x][destination.y].color == self.color {
            return false;
        }

        match self.value {
            PieceTypes::Pawn => self.valid_pawn_move(destination, gameboard),
            PieceTypes::Rook => self.valid_rook_move(destination, gameboard),
            PieceTypes::Knight => self.valid_knight_move(destination, gameboard),
            PieceTypes::Bishop => self.valid_bishop_move(destination, gameboard),
            PieceTypes::Queen => self.valid_queen_move(destination, gameboard),
            PieceTypes::King => self.valid_king_move(destination, gameboard),
            PieceTypes::Null => false,
        }
    }

    pub fn get_valid_moves(&self, gameboard: &GameBoard) -> Vec<PlayerMove> {
        let mut moves = Vec::new();

        let mut x = 0;
        let mut y = 0;

        while x < 8 {
            while y < 8 {
                let player_move = PlayerMove::new(x, y);
                if self.valid_move(&player_move, gameboard) {
                    moves.push(player_move.clone());
                }
                y += 1;
            }
            x += 1;
        }

        moves
    }

    fn valid_pawn_move(&self, destination: &PlayerMove, gameboard: &GameBoard) -> bool {
        let is_take_attempt = Piece::is_take_attempt(destination, gameboard);

        let is_diagonal = destination.x == self.x + 1 || destination.x == self.x - 1;
        let straight = destination.x == self.x;
        let is_one_forward = if self.color == Color::Black {
            destination.y == self.y - 1
        } else {
            destination.y == self.y + 1
        };

        let is_two_forward = if self.color == Color::Black {
            destination.y == self.y - 2
        } else {
            destination.y == self.y + 2
        };

        let start_pos = if self.color == Color::Black { 1 } else { 6 };

        if !is_take_attempt && self.y == start_pos {
            is_one_forward || is_two_forward && straight
        } else if !is_take_attempt {
            is_one_forward && straight
        } else {
            is_one_forward && is_diagonal
        }
    }

    fn valid_rook_move(&self, destination: &PlayerMove, gameboard: &GameBoard) -> bool {
        let dx = destination.x as isize - self.x as isize;
        let dy = destination.y as isize - self.y as isize;

        if dx != 0 && dy != 0 {
            return false;
        }

        if dx == 0 && dy == 0 {
            return false;
        }

        let step_x = dx.signum();
        let step_y = dy.signum();
        let steps = dx.abs().max(dy.abs());

        for i in 1..steps {
            let check_x = (self.x as isize + step_x * i) as usize;
            let check_y = (self.y as isize + step_y * i) as usize;

            if gameboard.board[check_x][check_y].value != PieceTypes::Null {
                return false;
            }
        }

        true
    }

    fn valid_knight_move(&self, destination: &PlayerMove, gameboard: &GameBoard) -> bool {
        let dx = (destination.x as isize - self.x as isize).abs();
        let dy = (destination.y as isize - self.y as isize).abs();

        matches!((dx, dy), (2, 1) | (1, 2))
    }

    fn valid_bishop_move(&self, destination: &PlayerMove, gameboard: &GameBoard) -> bool {
        let dx = destination.x as isize - self.x as isize;
        let dy = destination.y as isize - self.y as isize;

        if dx.abs() != dy.abs() || dx == 0 {
            return false;
        }

        let step_x = dx.signum();
        let step_y = dy.signum();
        let steps = dx.abs();

        for i in 1..steps {
            let check_x = (self.x as isize + step_x * i) as usize;
            let check_y = (self.y as isize + step_y * i) as usize;

            if gameboard.board[check_x][check_y].value != PieceTypes::Null {
                return false;
            }
        }

        true
    }

    fn valid_queen_move(&self, destination: &PlayerMove, gameboard: &GameBoard) -> bool {
        let dx: isize = (destination.x - self.x) as isize;
        let dy: isize = (destination.y - self.y) as isize;

        let is_diagonal = dx.abs() == dy.abs() && dx != 0;
        let is_straight = (dx == 0 && dy != 0) || (dy == 0 && dx != 0);

        if !is_diagonal && !is_straight {
            return false;
        }

        let step_x: isize = dx.signum();
        let step_y: isize = dy.signum();

        let steps = dx.abs().max(dy.abs());

        for i in 1..steps {
            let check_x = (self.x as isize + (step_x * i)) as usize;
            let check_y = (self.y as isize + (step_y * i)) as usize;

            if gameboard.board[check_x][check_y].value != PieceTypes::Null {
                return false;
            }
        }

        true
    }

    fn valid_king_move(&self, destination: &PlayerMove, _gameboard: &GameBoard) -> bool {
        let dx = (destination.x as isize - self.x as isize).abs();
        let dy = (destination.y as isize - self.y as isize).abs();

        dx <= 1 && dy <= 1 && (dx + dy) > 0
    }

    fn is_take_attempt(destination: &PlayerMove, gameboard: &GameBoard) -> bool {
        gameboard.board[destination.x][destination.y].value != PieceTypes::Null
    }
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
