#[derive(Clone, Copy, PartialEq, Eq)]
enum Pieces {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Color {
    White,
    Black,
}

#[derive(Clone, Copy)]
struct Position {
    x: u8,
    y: u8,
}

#[derive(Clone, Copy)]
struct ChessPiece {
    piece_type: Pieces,
    color: Color,
}

#[derive(Clone, Copy)]
struct ChessBoard {
    squares: [Option<ChessPiece>; 64],
}

impl Position {
    const fn to_index(&self) -> u8 {
        self.y * 8 + self.x
    }

    const fn from_index(index: u8) -> Self {
        Self {
            x: index % 8,
            y: index / 8,
        }
    }

    const fn is_valid(&self) -> bool {
        self.x < 8 && self.y < 8
    }
}

impl ChessPiece {
    const fn new(piece_type: Pieces, color: Color) -> Self {
        Self { piece_type, color }
    }

    const fn get_piece_type(&self) -> Pieces {
        self.piece_type
    }
}

impl ChessBoard {
    const fn new() -> Self {
        Self {
            squares: [None; 64],
        }
    }

    const fn get_piece(&self, index: u8) -> Option<&ChessPiece> {
        self.squares.get(index as usize)?.as_ref()
    }

    fn force_move(&mut self, from: u8, to: u8) -> bool {
        if from >= 64 || to >= 64 || from == to {
            return false;
        }

        let Some(piece) = self.squares[from as usize].take() else {
            return false;
        };

        self.squares[to as usize] = Some(piece);
        true
    }

    fn set_piece(&mut self, index: u8, piece: ChessPiece) -> bool {
        if index >= 64 {
            return false;
        }

        self.squares[index as usize] = Some(piece);
        true
    }

    fn remove_piece(&mut self, index: u8) -> Option<ChessPiece> {
        self.squares.get_mut(index as usize)?.take()
    }
}

fn main() {
    println!("Hello, world!");
}
