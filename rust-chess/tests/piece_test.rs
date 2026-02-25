#[cfg(test)]
mod tests {
    use chess::gameboard::GameBoard;
    use chess::piece::{Piece, PieceTypes, PlayerMove};
    use colored::Color;

    fn empty_board() -> GameBoard {
        GameBoard::default()
    }

    fn make_move(x: usize, y: usize) -> PlayerMove {
        PlayerMove::new(x, y)
    }

    fn place_piece(board: &mut GameBoard, x: usize, y: usize, piece_type: PieceTypes) {
        board.board[x][y] = Piece::new(Color::White, piece_type, x, y);
    }

    // FAIL
    #[test]
    fn rook_valid_vertical_move() {
        let board = empty_board();
        let rook = Piece::new(Color::White, PieceTypes::Rook, 4, 4);
        assert!(!rook.valid_move(&make_move(4, 7), &board));
    }

    #[test]
    fn rook_valid_horizontal_move() {
        let board = empty_board();
        let rook = Piece::new(Color::White, PieceTypes::Rook, 4, 4);
        assert!(rook.valid_move(&make_move(7, 4), &board));
    }

    #[test]
    fn rook_blocked_by_piece() {
        let mut board = empty_board();
        let rook = Piece::new(Color::White, PieceTypes::Rook, 4, 4);
        place_piece(&mut board, 4, 6, PieceTypes::Pawn);
        assert!(!rook.valid_move(&make_move(4, 7), &board));
    }

    #[test]
    fn rook_rejects_diagonal_move() {
        let board = empty_board();
        let rook = Piece::new(Color::White, PieceTypes::Rook, 4, 4);
        assert!(!rook.valid_move(&make_move(6, 6), &board));
    }

    #[test]
    fn bishop_valid_diagonal_move() {
        let board = empty_board();
        let bishop = Piece::new(Color::White, PieceTypes::Bishop, 4, 4);
        assert!(bishop.valid_move(&make_move(6, 6), &board));
    }

    #[test]
    fn bishop_rejects_straight_move() {
        let board = empty_board();
        let bishop = Piece::new(Color::White, PieceTypes::Bishop, 4, 4);
        assert!(!bishop.valid_move(&make_move(4, 6), &board));
    }

    #[test]
    fn bishop_blocked_by_piece() {
        let mut board = empty_board();
        let bishop = Piece::new(Color::White, PieceTypes::Bishop, 4, 4);
        place_piece(&mut board, 5, 5, PieceTypes::Pawn);
        assert!(!bishop.valid_move(&make_move(6, 6), &board));
    }

    #[test]
    fn knight_valid_l_shape_moves() {
        let board = empty_board();
        let knight = Piece::new(Color::White, PieceTypes::Knight, 4, 4);
        assert!(knight.valid_move(&make_move(6, 5), &board));
        assert!(knight.valid_move(&make_move(6, 3), &board));
        assert!(knight.valid_move(&make_move(5, 6), &board));
        assert!(knight.valid_move(&make_move(3, 6), &board));
    }

    #[test]
    fn knight_rejects_straight_move() {
        let board = empty_board();
        let knight = Piece::new(Color::White, PieceTypes::Knight, 4, 4);
        assert!(!knight.valid_move(&make_move(4, 6), &board));
    }

    #[test]
    fn knight_jumps_over_pieces() {
        let mut board = empty_board();
        let knight = Piece::new(Color::White, PieceTypes::Knight, 4, 4);
        place_piece(&mut board, 4, 5, PieceTypes::Pawn);
        place_piece(&mut board, 5, 5, PieceTypes::Pawn);
        assert!(knight.valid_move(&make_move(6, 5), &board));
    }

    #[test]
    fn queen_invalid_straight_move() {
        let board = empty_board();
        let queen = Piece::new(Color::White, PieceTypes::Queen, 4, 4);
        assert!(!queen.valid_move(&make_move(4, 7), &board));
    }

    #[test]
    fn queen_valid_straight_move() {
        let board = empty_board();
        let queen = Piece::new(Color::White, PieceTypes::Queen, 4, 4);
        assert!(queen.valid_move(&make_move(6, 4), &board));
    }

    #[test]
    fn queen_valid_diagonal_move() {
        let board = empty_board();
        let queen = Piece::new(Color::White, PieceTypes::Queen, 4, 4);
        assert!(queen.valid_move(&make_move(6, 6), &board));
    }

    #[test]
    fn queen_blocked_diagonally() {
        let mut board = empty_board();
        let queen = Piece::new(Color::White, PieceTypes::Queen, 4, 4);
        place_piece(&mut board, 5, 5, PieceTypes::Pawn);
        assert!(!queen.valid_move(&make_move(6, 6), &board));
    }

    #[test]
    fn queen_rejects_invalid_move() {
        let board = empty_board();
        let queen = Piece::new(Color::White, PieceTypes::Queen, 4, 4);
        assert!(!queen.valid_move(&make_move(6, 7), &board));
    }

    #[test]
    fn king_valid_one_step_moves() {
        let board = empty_board();
        let king = Piece::new(Color::White, PieceTypes::King, 4, 4);
        assert!(king.valid_move(&make_move(4, 5), &board));
        assert!(king.valid_move(&make_move(5, 4), &board));
        assert!(king.valid_move(&make_move(5, 5), &board));
    }

    // FAILED
    #[test]
    fn king_rejects_two_step_move() {
        let board = empty_board();
        let king = Piece::new(Color::White, PieceTypes::King, 4, 4);
        assert!(!king.valid_move(&make_move(4, 6), &board));
    }
}
