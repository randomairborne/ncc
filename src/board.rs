use std::fmt::Display;

pub struct Board {
    inner: RawBoard,
}

impl Board {
    const BLANK_RANK: RawRank = [None; 8];
    const EIGHTH_RANK: RawRank = [
        Some((Color::Black, Piece::Rook)),
        Some((Color::Black, Piece::Knight)),
        Some((Color::Black, Piece::Bishop)),
        Some((Color::Black, Piece::Queen)),
        Some((Color::Black, Piece::King)),
        Some((Color::Black, Piece::Bishop)),
        Some((Color::Black, Piece::Knight)),
        Some((Color::Black, Piece::Rook)),
    ];
    const FIRST_RANK: RawRank = [
        Some((Color::White, Piece::Rook)),
        Some((Color::White, Piece::Knight)),
        Some((Color::White, Piece::Bishop)),
        Some((Color::White, Piece::Queen)),
        Some((Color::White, Piece::King)),
        Some((Color::White, Piece::Bishop)),
        Some((Color::White, Piece::Knight)),
        Some((Color::White, Piece::Rook)),
    ];
    const SECOND_RANK: RawRank = [Some((Color::White, Piece::Pawn)); 8];
    const SEVENTH_RANK: RawRank = [Some((Color::Black, Piece::Pawn)); 8];

    pub fn new() -> Self {
        let inner = [
            Self::FIRST_RANK,
            Self::SECOND_RANK,
            Self::BLANK_RANK,
            Self::BLANK_RANK,
            Self::BLANK_RANK,
            Self::BLANK_RANK,
            Self::SEVENTH_RANK,
            Self::EIGHTH_RANK,
        ];
        Self { inner }
    }

    pub fn raw(&self) -> RawBoard {
        self.inner
    }

    pub fn make_move(&mut self, from: BoardSpace, to: BoardSpace) {
        let to_rank = to.rank as usize;
        let to_file = to.file as usize;
        let from_rank = from.rank as usize;
        let from_file = from.file as usize;
        self.inner[to_rank][to_file] = self.inner[from_rank][from_file];
        self.inner[from_rank][from_file] = None;
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Rank {
    First = 0,
    Second = 1,
    Third = 2,
    Fourth = 3,
    Fifth = 4,
    Sixth = 5,
    Seventh = 6,
    Eighth = 7,
}

#[derive(Debug, Clone, Copy)]
pub enum File {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
    H = 7,
}

impl Rank {
    pub fn from_char(c: char) -> Result<Self, ()> {
        let v = match c {
            '1' => Rank::First,
            '2' => Rank::Second,
            '3' => Rank::Third,
            '4' => Rank::Fourth,
            '5' => Rank::Fifth,
            '6' => Rank::Sixth,
            '7' => Rank::Seventh,
            '8' => Rank::Eighth,
            _ => return Err(()),
        };
        Ok(v)
    }
}

impl File {
    pub fn from_char(c: char) -> Result<Self, ()> {
        let file = match c {
            'A' | 'a' => Self::A,
            'B' | 'b' => Self::B,
            'C' | 'c' => Self::C,
            'D' | 'd' => Self::D,
            'E' | 'e' => Self::E,
            'F' | 'f' => Self::F,
            'G' | 'g' => Self::G,
            'H' | 'h' => Self::H,
            _ => return Err(()),
        };
        Ok(file)
    }
}

pub struct BoardSpace {
    pub rank: Rank,
    pub file: File,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum Color {
    Black,
    White,
}

impl Color {
    pub fn flip(&mut self) {
        *self = self.flipped();
    }

    pub fn flipped(&self) -> Self {
        match self {
            Self::Black => Self::White,
            Self::White => Self::Black,
        }
    }

    pub fn owo(&self) -> owo_colors::AnsiColors {
        match self {
            Color::Black => owo_colors::AnsiColors::Black,
            Color::White => owo_colors::AnsiColors::White,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum Piece {
    King,
    Queen,
    Rook,
    Knight,
    Bishop,
    Pawn,
}

impl Piece {
    pub fn display(self, color: Color) -> char {
        match color {
            Color::Black => self.display_black(),
            Color::White => self.display_white(),
        }
    }

    pub fn display_black(self) -> char {
        match self {
            Piece::King => '♔',
            Piece::Queen => '♕',
            Piece::Rook => '♖',
            Piece::Knight => '♘',
            Piece::Bishop => '♗',
            Piece::Pawn => '♙',
        }
    }

    pub fn display_white(self) -> char {
        match self {
            Piece::King => '♚',
            Piece::Queen => '♛',
            Piece::Rook => '♜',
            Piece::Knight => '♞',
            Piece::Bishop => '♝',
            Piece::Pawn => '♟',
        }
    }
}

pub type RawBoard = [RawRank; 8];
type RawRank = [RawSquare; 8];
type RawSquare = Option<(Color, Piece)>;
