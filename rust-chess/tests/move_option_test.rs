#[cfg(test)]
mod tests {
    use chess::gameboard::GameBoard;

    fn empty_board() -> GameBoard {
        GameBoard::default()
    }

    #[test]
    fn rook_valid_moves() {
        let board = empty_board();
        let rook = board.board[0][0];
        let moves = rook.get_valid_moves(&board);

        for x in &moves {
            println!("{}, {}", x.x, x.y);
        }

        assert!(moves.is_empty());
    }
}
