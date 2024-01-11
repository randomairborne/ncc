use std::fmt::Write;

use owo_colors::OwoColorize;

use crate::board::{Board, Color, RawBoard};

impl Board {
    pub fn display(&self, player_color: Color) -> Result<String, std::fmt::Error> {
        let mut b = String::with_capacity(9 * 9 * 3 * 3);
        let mut square_color = player_color;
        let (rank_delta, mut rank_num) = match player_color {
            Color::White => (-1, 8),
            Color::Black => (1, 1),
        };
        let target = match player_color {
            Color::White => self.raw(),
            Color::Black => self.flipped_raw(),
        };
        Self::write_files(&mut b, player_color)?;
        for rank in target.iter().rev() {
            write!(b, "{rank_num} ")?;
            for square in rank {
                if let Some((piece_color, piece)) = square {
                    let piece = format!(" {} ", piece.display(*piece_color));
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

    fn flipped_raw(&self) -> RawBoard {
        let mut flipped = self.raw().clone();
        flipped.reverse();
        for item in flipped.iter_mut() {
            item.reverse()
        }
        flipped
    }

    fn write_files(b: &mut String, color: Color) -> Result<(), std::fmt::Error> {
        match color {
            Color::White => writeln!(b, "   a  b  c  d  e  f  g  h"),
            Color::Black => writeln!(b, "   h  g  f  e  d  c  b  a"),
        }
    }
}
