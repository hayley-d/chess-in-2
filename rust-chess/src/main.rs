use chess::gameboard::*;
use chess::piece::*;
use colored::Color;
use rand::prelude::IndexedRandom;
use rand::seq::SliceRandom;

fn main() {
    let gameboard = GameBoard::new();

    game_loop(&gameboard);
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
