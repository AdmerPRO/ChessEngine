mod board;
mod pieces;

use board::{ChessBoard, Color, Position};
use pieces::{ChessPiece, Pieces};

fn main() {
    let mut chess_board = ChessBoard::new();
    chess_board.set_piece(
        Position { x: 0, y: 0 }.to_index(),
        ChessPiece::new(Pieces::Rook, Color::White),
    );
    chess_board.print_pieces();
}
