use crate::board::Board;
use crate::Error;

impl Board {
    pub fn turn(&mut self, code: String) -> Result<(), Error> {
        let chars: Vec<char> = code.chars().collect();
        let length = chars.len();
        if length != 4 && length != 5 {
            return Err(Error::BadNotationLength);
        }
        let mut base_idx = 0;
        if length == 5 {
            base_idx = 1;
        }
        let start = &chars[base_idx..=base_idx + 1];
        let end = &chars[base_idx + 2..=base_idx + 3];
        println!("{start:?} {end:?}");
        Ok(())
    }
}
