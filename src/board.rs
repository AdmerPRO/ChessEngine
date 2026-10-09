use crate::pieces::ChessPiece;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

#[derive(Clone, Copy)]
pub struct Position {
    pub x: u8,
    pub y: u8,
}

#[derive(Clone, Copy)]
pub struct ChessBoard {
    squares: [Option<ChessPiece>; 64],
}

impl Position {
    pub fn to_index(&self) -> u8 {
        self.y * 8 + self.x
    }

    pub fn from_index(index: u8) -> Self {
        Self {
            x: index % 8,
            y: index / 8,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.x < 8 && self.y < 8
    }
}

impl ChessBoard {
    pub fn new() -> Self {
        Self {
            squares: [None; 64],
        }
    }

    pub fn get_piece(&self, index: u8) -> Option<&ChessPiece> {
        self.squares.get(index as usize)?.as_ref()
    }

    pub fn force_move(&mut self, from: u8, to: u8) -> bool {
        if from >= 64 || to >= 64 || from == to {
            return false;
        }

        let Some(piece) = self.squares[from as usize].take() else {
            return false;
        };

        self.squares[to as usize] = Some(piece);
        true
    }

    pub fn set_piece(&mut self, index: u8, piece: ChessPiece) -> bool {
        if index >= 64 {
            return false;
        }

        self.squares[index as usize] = Some(piece);
        true
    }

    pub fn remove_piece(&mut self, index: u8) -> Option<ChessPiece> {
        self.squares.get_mut(index as usize)?.take()
    }

    pub fn print_pieces(&self) {
        for (index, square) in self.squares.iter().enumerate() {
            if let Some(piece) = square {
                println!("Index: {}, Piece: {:?}", index, piece);
            }
        }
    }
}