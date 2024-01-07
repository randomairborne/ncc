use owo_colors::{DynColors, OwoColorize};
use std::fmt::{Display, Formatter, Write};

pub struct Board {
    inner: RawBoard,
}

impl Board {
    const FIRST_RANK: Rank = [
        Some((Color::White, Piece::Rook)),
        Some((Color::White, Piece::Knight)),
        Some((Color::White, Piece::Bishop)),
        Some((Color::White, Piece::King)),
        Some((Color::White, Piece::Queen)),
        Some((Color::White, Piece::Bishop)),
        Some((Color::White, Piece::Knight)),
        Some((Color::White, Piece::Rook)),
    ];
    const SECOND_RANK: Rank = [Some((Color::White, Piece::Pawn)); 8];
    const BLANK_RANK: Rank = [None; 8];
    const SEVENTH_RANK: Rank = [Some((Color::Black, Piece::Pawn)); 8];
    const EIGHTH_RANK: Rank = [
        Some((Color::Black, Piece::Rook)),
        Some((Color::Black, Piece::Knight)),
        Some((Color::Black, Piece::Bishop)),
        Some((Color::Black, Piece::King)),
        Some((Color::Black, Piece::Queen)),
        Some((Color::Black, Piece::Bishop)),
        Some((Color::Black, Piece::Knight)),
        Some((Color::Black, Piece::Rook)),
    ];
    pub fn new() -> Self {
        let inner = [
            Self::EIGHTH_RANK,
            Self::SEVENTH_RANK,
            Self::BLANK_RANK,
            Self::BLANK_RANK,
            Self::BLANK_RANK,
            Self::BLANK_RANK,
            Self::SECOND_RANK,
            Self::FIRST_RANK,
        ];
        Self { inner }
    }
    pub fn display(&self, player_color: Color) -> Result<String, std::fmt::Error> {
        let mut b = String::with_capacity(9 * 9 * 3 * 3);
        let mut square_color = player_color;
        let (rank_delta, mut rank_num) = match player_color {
            Color::White => (-1, 8),
            Color::Black => (1, 1),
        };
        let target = match player_color {
            Color::White => self.inner,
            Color::Black => self.flipped().inner,
        };
        Self::write_files(&mut b, player_color)?;
        for rank in target {
            write!(b, "{rank_num} ")?;
            for square in rank {
                if let Some((piece_color, piece)) = square {
                    let piece = format!(" {} ", piece.display(piece_color));
                    write!(b, "{}", piece.on_color(square_color.owo()))?;
                } else {
                    let blank = "   ".on_color(square_color.owo());
                    write!(b, "{blank}")?;
                }
                square_color.flip();
            }
            write!(b, " {rank_num}")?;
            rank_num += rank_delta;
            square_color.flip();
            writeln!(b)?;
        }
        Self::write_files(&mut b, player_color)?;
        Ok(b)
    }

    fn flipped(&self) -> Self {
        let mut flipped = self.inner.clone();
        flipped.reverse();
        for item in flipped.iter_mut() {
            item.reverse()
        }
        Self { inner: flipped }
    }

    fn write_files(b: &mut String, color: Color) -> Result<(), std::fmt::Error> {
        match color {
            Color::White => writeln!(b, "   a  b  c  d  e  f  g  h"),
            Color::Black => writeln!(b, "   h  g  f  e  d  c  b  a"),
        }
    }
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

type RawBoard = [Rank; 8];
type Rank = [Square; 8];
type Square = Option<(Color, Piece)>;
