use crate::board::{Board, BoardSpace, Color, File, Rank};

fn notation_to_position(data: &[char]) -> Result<BoardSpace, NotationParseError> {
    if data.len() != 2 {
        return Err(NotationParseError::BadFragmentLength);
    }
    let file = File::from_char(data[0]).map_err(|_| NotationParseError::UnknownFile)?;
    let rank = Rank::from_char(data[1]).map_err(|_| NotationParseError::UnknownRank)?;

    Ok(BoardSpace { rank, file })
}

#[derive(Debug, thiserror::Error)]
pub enum NotationParseError {
    #[error("Your chess notation was a weird length.")]
    BadLength,
    #[error("Your fragmented chess notation was a weird length.")]
    BadFragmentLength,
    #[error("That rank doesn't exist.")]
    UnknownRank,
    #[error("That file doesn't exist.")]
    UnknownFile,
}

impl Board {
    pub fn turn(&mut self, code: String) -> Result<(), NotationParseError> {
        let chars: Vec<char> = code.chars().collect();
        let length = chars.len();
        if length != 4 && length != 5 {
            return Err(NotationParseError::BadLength);
        }
        let mut base_idx = 0;
        if length == 5 {
            base_idx = 1;
        }
        let start = &chars[base_idx..=base_idx + 1];
        let end = &chars[base_idx + 2..=base_idx + 3];
        let start_pos = notation_to_position(start)?;
        let end_pos = notation_to_position(end)?;
        self.make_move(start_pos, end_pos);
        Ok(())
    }

    pub fn has_mate(&self, color: Color) -> bool {
        false
    }
}
