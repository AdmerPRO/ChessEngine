use crate::board::Color;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pieces {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Clone, Copy, Debug)]
pub struct ChessPiece {
    pub piece_type: Pieces,
    pub color: Color,
}

impl ChessPiece {
    pub fn new(piece_type: Pieces, color: Color) -> Self {
        Self { piece_type, color }
    }

    pub fn get_piece_type(&self) -> Pieces {
        self.piece_type
    }
}