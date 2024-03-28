use std::cmp::min;
use std::fmt::Display;

use crate::error::IndexError;


/// Represents a Canvas that is drawn by braille characters.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Canvas {
    x: usize,
    y: usize,
    char_x: usize,
    char_y: usize,
    data: Vec<u32>,
}


impl Canvas {
    const VALUES: [u32; 8] = [1, 2, 4, 64, 8, 16, 32, 128];
    const VALUES_INVERTED: [u32; 8] = [!1, !2, !4, !64, !8, !16, !32, !128];

    /// Returns the index of `self.data` the given xy-coords lie in.
    /// Returns `None` if out of range.
    fn coords_to_index(&self, x: usize, y: usize) -> Option<usize> {
        let res = x / 2 + self.char_x * (y / 4);

        return if res >= self.data.len() {
            None
        } else {
            Some(res)
        };
    }

    /// Creates a new Canvas sized 0 by 0.
    /// Changing size currently not supported. Use `with_size()` instead.
    pub fn new() -> Self {
        Canvas {
            x: 0,
            y: 0,
            char_x: 0,
            char_y: 0,
            data: vec![],
        }
    }

    /// Creates a new Canvas with size `x` by `y`.
    /// `x` and `y` correspond to the singular points, not the amount of symbols.
    /// Meaning x=4 and y=4 Will only contain two characters,
    /// since every character can only display 2*4 points.
    pub fn with_size(x: usize, y: usize) -> Self {
        let char_x = x / 2 + min(x % 2, 1);
        let char_y = y / 4 + min(y % 4, 1);

        Canvas {
            x,
            y,
            char_x,
            char_y,
            data: vec![0u32; char_x * char_y],
        }
    }

    /// Flips the point at a given x, y position. 0, 0 is top left.
    /// If coordinates are out of range returns an `IndexError` otherwise `OK()`.
    pub fn flip(&mut self, x: usize, y: usize) -> Result<(), IndexError> {
        return if let Some(i) = self.coords_to_index(x, y) {
            self.data[i] ^= Canvas::VALUES[(y % 4) + (x % 2) * 4];

            Ok(())
        } else {
            Err(IndexError::USizeMatrix(self.x, self.y, x, y))
        };
    }

    /// Resets the point at a given x, y position. 0, 0 is top left.
    /// If coordinates are out of range returns an `IndexError` otherwise `OK()`.
    pub fn reset(&mut self, x: usize, y: usize) -> Result<(), IndexError> {
        return if let Some(i) = self.coords_to_index(x, y) {
            self.data[i] &= Canvas::VALUES_INVERTED[(y % 4) + (x % 2) * 4];

            Ok(())
        } else {
            Err(IndexError::USizeMatrix(self.x, self.y, x, y))
        };
    }

    /// Resets the whole canvas.
    pub fn reset_all(&mut self) {
        self.data = self.data.iter().map(|&v| v & !0b11111111).collect();
    }

    /// Sets the point at a given x, y position. 0, 0 is top left.
    /// If coordinates are out of range returns an `IndexError` otherwise `OK()`.
    pub fn set(&mut self, x: usize, y: usize) -> Result<(), IndexError> {
        return if let Some(i) = self.coords_to_index(x, y) {
            self.data[i] |= Canvas::VALUES[(y % 4) + (x % 2) * 4];

            Ok(())
        } else {
            Err(IndexError::USizeMatrix(self.x, self.y, x, y))
        };
    }
}


impl Display for Canvas {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = String::with_capacity(self.x * self.y);

        for (i, value) in self.data.iter().enumerate() {
            string.push(char::from_u32(0x00002800 | (value & 0b11111111u32)).unwrap());

            if i % self.char_x == self.char_x - 1 {
                string.push('\n');
            };
        }

        write!(f, "{}", string.trim())
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn canvas_with_size() {
        let c = Canvas::with_size(3, 6);
        dbg!(c.to_string());

        let c = Canvas::with_size(2, 4);
        dbg!(c.to_string());

        let c = Canvas::with_size(4, 4);
        dbg!(c.to_string());

        let c = Canvas::with_size(7, 10);
        dbg!(c);
    }

    #[test]
    fn canvas_set() {
        let mut c = Canvas::with_size(7, 10);
        println!("{c} {} {}", c.char_x, c.char_y);

        c.set(0, 0).unwrap();
        c.set(6, 9).unwrap();
        c.set(6, 0).unwrap();
        c.set(0, 9).unwrap();
        println!("{c}");
    }

    #[test]
    fn canvas_reset() {
        let mut c = Canvas::with_size(7, 10);
        println!("{c} {} {}", c.char_x, c.char_y);

        c.set(0, 0).unwrap();
        c.set(6, 9).unwrap();
        c.set(6, 0).unwrap();
        c.set(0, 9).unwrap();
        println!("{c}");

        c.reset(0, 0).unwrap();
        println!("{c}");
        c.reset(6, 9).unwrap();
        println!("{c}");
        c.reset(6, 0).unwrap();
        println!("{c}");
        c.reset(0, 9).unwrap();
        println!("{c}");
    }

    #[test]
    fn canvas_reset_all() {
        let mut c = Canvas::with_size(7, 10);
        println!("{c} {} {}", c.char_x, c.char_y);

        c.set(0, 0).unwrap();
        c.set(6, 9).unwrap();
        c.set(6, 0).unwrap();
        c.set(0, 9).unwrap();
        println!("{c}");

        c.reset_all();
        println!("{c}");
    }

    #[test]
    fn canvas_flip() {
        let mut c = Canvas::with_size(7, 10);
        println!("{c} {} {}", c.char_x, c.char_y);

        c.flip(0, 0).unwrap();
        c.flip(6, 9).unwrap();
        c.flip(6, 0).unwrap();
        c.flip(0, 9).unwrap();
        println!("{c}");

        c.flip(0, 0).unwrap();
        c.flip(6, 9).unwrap();
        c.flip(6, 0).unwrap();
        c.flip(0, 9).unwrap();
        println!("{c}");
    }
}