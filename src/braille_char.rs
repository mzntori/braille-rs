use std::fmt::{Display, Formatter};
use std::ops::Not;

use crate::error::IndexError;


/// Represents a braille character.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct BrailleChar {
    data: u32,
}


impl BrailleChar {
    const VALUES: [u32; 8] = [1, 2, 4, 64, 8, 16, 32, 128];
    const VALUES_INVERTED: [u32; 8] = [!1, !2, !4, !64, !8, !16, !32, !128];

    /// Creates a new braille character that is blank.
    pub fn new() -> Self {
        BrailleChar {
            data: 0,
        }
    }

    /// Creates a new braille character with the given data.
    ///
    /// LSB is top left and MSB is bottom right. However, it goes up in the following order,
    /// where 0 is the LSB and 7 the MSB:
    ///
    /// `(0) (3)`
    ///
    /// `(1) (4)`
    ///
    /// `(2) (5)`
    ///
    /// `(6) (7)`
    ///
    pub fn with_data(data: u8) -> Self {
        BrailleChar {
            data: data as u32,
        }
    }

    /// Takes an u32 and replaces the data with it.
    /// Only the lowest 8 bit currently effect the outcome.
    /// The other 24 are reserved for colors in case I ever add them.
    ///
    /// LSB is top left and MSB is bottom right. However, it goes up in the following order,
    /// where 0 is the LSB and 7 the MSB:
    ///
    /// `(0) (3)`
    ///
    /// `(1) (4)`
    ///
    /// `(2) (5)`
    ///
    /// `(6) (7)`
    ///
    /// Meaning `0b00111010` would result in
    ///
    pub fn set_data(&mut self, data: u32) {
        self.data = data;
    }

    /// Takes an u32 and performs logical or on the data.
    fn or_data(&mut self, data: u32) {
        self.data |= data;
    }

    /// Takes an u32 and performs logical xor on the data.
    fn xor_data(&mut self, data: u32) {
        self.data ^= data;
    }

    /// Takes an u32 and performs logical and on the data
    fn and_data(&mut self, data: u32) {
        self.data &= data.not();
    }

    /// Sets the point at a given x, y position. 0, 0 is top left.
    /// If x or y are out of range returns an `IndexError`.
    pub fn set(&mut self, x: usize, y: usize) -> Result<(), IndexError> {
        let i = y + 4 * x;

        return if i < 8 {
            self.or_data(BrailleChar::VALUES[y + 4 * x]);
            Ok(())
        } else {
            Err(IndexError::USizeMatrix(2, 4, x, y))
        }

    }

    /// Sets all points.
    pub fn set_all(&mut self) {
        self.set_data(0b11111111);
    }

    /// Resets the point at a given x, y position. 0, 0 is top left.
    /// If x or y are out of range returns an `IndexError`.
    pub fn reset(&mut self, x: usize, y: usize) -> Result<(), IndexError> {
        let i = y + 4 * x;

        return if i < 8 {
            self.and_data(BrailleChar::VALUES_INVERTED[y + 4 * x]);
            Ok(())
        } else {
            Err(IndexError::USizeMatrix(2, 4, x, y))
        }
    }

    /// Resets all points.
    pub fn reset_all(&mut self) {
        self.data = 0;
    }

    /// Clears the point at a given x, y position. 0, 0 is top left.
    /// If x or y are out of range returns an `IndexError`.
    pub fn flip(&mut self, x: usize, y: usize) -> Result<(), IndexError> {
        let i = y + 4 * x;

        return if i < 8 {
            self.xor_data(BrailleChar::VALUES[y + 4 * x]);
            Ok(())
        } else {
            Err(IndexError::USizeMatrix(2, 4, x, y))
        }
    }

    /// Flips all points.
    pub fn flip_all(&mut self) {
        self.data ^= 0b11111111;
    }
}


impl Display for BrailleChar {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", char::from(self))
    }
}


impl From<BrailleChar> for char {
    fn from(value: BrailleChar) -> Self {
        char::from(&value)
    }
}


impl From<&BrailleChar> for char {
    fn from(value: &BrailleChar) -> Self {
        char::from_u32(0x00002800 | (value.data & 0b11111111u32)).unwrap()
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn braille_character_single() {
        let mut c = BrailleChar::new();

        c.set(0, 2).unwrap();
        c.set(1, 3).unwrap();
        assert_eq!(c, BrailleChar::with_data(0b10000100));

        c.flip(0, 2).unwrap();
        c.flip(1, 2).unwrap();
        c.flip(0, 1).unwrap();
        assert_eq!(c, BrailleChar::with_data(0b10100010));

        c.reset(0, 1).unwrap();
        c.reset(1, 2).unwrap();
        c.reset(1, 3).unwrap();
        assert_eq!(c, BrailleChar::new());
    }

    #[test]
    fn braille_character_all() {
        let mut c = BrailleChar::new();

        c.set_all();
        assert_eq!(c, BrailleChar::with_data(255));

        c.reset_all();
        assert_eq!(c, BrailleChar::with_data(0));

        c.flip_all();
        assert_eq!(c, BrailleChar::with_data(255));
    }
}