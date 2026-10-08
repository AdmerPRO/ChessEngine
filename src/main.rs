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

struct ChessPiece {
    piece_type: Pieces,
    color: Color,
    place: u8,
}

#[derive(Clone, Copy)]
struct ChessBoard {
    squares: [Option<ChessPiece>; 64],
}

impl ChessPiece {
    const fn new(piece_type: Pieces, color: Color, place: u8) -> Self {
        ChessPiece {
            piece_type,
            color,
            place,
        }
    }

    const fn get_piece_type(&self) -> Pieces {
        self.piece_type
    }

    const fn index_to_position(&self) -> Position {
        let x = self.place % 8;
        let y = self.place / 8;
        Position { x, y }
    }

    const fn position_to_index(position: Position) -> u8 {
        position.y * 8 + position.x
    }

    const fn is_valid(&self) -> bool {
        self.place < 64
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
}

fn main() {
    println!("Hello, world!");
}
