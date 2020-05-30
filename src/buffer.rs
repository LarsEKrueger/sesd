/*
    MIT License

    Copyright (c) 2020 Lars Krueger <lars_e_krueger@gmx.de>

    Permission is hereby granted, free of charge, to any person obtaining a copy
    of this software and associated documentation files (the "Software"), to deal
    in the Software without restriction, including without limitation the rights
    to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
    copies of the Software, and to permit persons to whom the Software is
    furnished to do so, subject to the following conditions:

    The above copyright notice and this permission notice shall be included in all
    copies or substantial portions of the Software.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
    AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
    OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
    SOFTWARE.
*/

//! Edit buffer

pub struct Buffer<T> {
    /// Buffer of tokens
    tokens: Vec<T>,

    /// Cursor as an index into `tokens`.
    ///
    /// Range: [0, tokens.len()]
    cursor: usize,
}

impl<T> Buffer<T> {
    pub fn new() -> Self {
        Self {
            tokens: Vec::new(),
            cursor: 0,
        }
    }

    /// Search from the given position forward through the tokens until the predicate becomes true.
    ///
    /// If the given position is invalid, None will be returned.
    ///
    /// Return None, if the index wasn't found. Otherwise, return the index at which the predicate
    /// became true.
    pub fn search_forward<F>(&self, start: usize, mut until: F) -> Option<usize>
    where
        F: FnMut(&Vec<T>, usize) -> bool,
    {
        let mut index = start;
        while index < self.tokens.len() {
            if until(&self.tokens, index) {
                return Some(index);
            }
            index += 1;
        }

        None
    }

    /// Move the cursor forward by a fixed number of tokens
    pub fn move_forward(&mut self, steps: usize) {
        if steps + self.cursor <= self.tokens.len() {
            self.cursor += steps;
        } else {
            self.cursor = self.tokens.len();
        }
    }

    /// Move cursor to the end of the token list
    pub fn move_end(&mut self) {
        self.cursor = self.tokens.len();
    }

    /// Move to the start of the token list
    pub fn move_start(&mut self) {
        self.cursor = 0;
    }

    /// Move the cursor forward until the predicate becomes true
    pub fn skip_forward<F>(&mut self, until: F)
    where
        F: FnMut(&Vec<T>, usize) -> bool,
    {
        if let Some(index) = self.search_forward(self.cursor, until) {
            self.cursor = index;
        }
    }

    /// Enter a single token at the current cursor position and advance the cursor.
    ///
    /// This will insert the token.
    ///
    /// Later extensions might also overwrite, depending on settings
    pub fn enter(&mut self, t: T) {
        self.tokens.insert(self.cursor, t);
        self.cursor += 1;
    }

    /// Delete tokens at the cursor
    pub fn delete(&mut self, n: usize) {
        self.tokens.drain(self.cursor..(self.cursor + n));
    }
}

impl<T> Buffer<T>
where
    T: Clone,
{
    /// Enter a slice of tokens
    ///
    /// This will insert the tokens.
    ///
    /// Later extensions might also overwrite, depending on settings
    pub fn enter_slice(&mut self, tokens: &[T]) {
        self.tokens.reserve(tokens.len());
        for t in tokens {
            self.enter(t.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search() {
        let mut buffer = Buffer::<u32>::new();
        buffer.tokens.push(3);
        buffer.tokens.push(1);
        buffer.tokens.push(4);
        buffer.tokens.push(5);

        assert_eq!(buffer.search_forward(0, |b, x| b[x] == 4), Some(2));
        assert_eq!(buffer.search_forward(2, |b, x| b[x] == 4), Some(2));
        assert_eq!(buffer.search_forward(3, |b, x| b[x] == 4), None);
        assert_eq!(buffer.search_forward(4, |b, x| b[x] == 4), None);
        assert_eq!(buffer.search_forward(0, |b, x| b[x] == 8), None);
    }

    #[test]
    fn move_cursor() {
        let mut buffer = Buffer::<u32>::new();
        buffer.tokens.push(3);
        buffer.tokens.push(1);
        buffer.tokens.push(4);
        buffer.tokens.push(5);

        assert_eq!(buffer.cursor, 0);

        buffer.move_forward(1);
        assert_eq!(buffer.cursor, 1);

        buffer.move_forward(40);
        assert_eq!(buffer.cursor, 4);
    }

    #[test]
    fn enter() {
        let mut buffer = Buffer::<u32>::new();
        buffer.enter_slice(&[3, 1, 4, 5]);
        assert_eq!(buffer.tokens.len(), 4);
        assert_eq!(buffer.cursor, 4);

        buffer.move_start();
        buffer.move_forward(2);
        assert_eq!(buffer.cursor, 2);

        buffer.enter_slice(&[8, 7, 6]);
        assert_eq!(buffer.cursor, 5);
        assert_eq!(buffer.tokens, &[3, 1, 8, 7, 6, 4, 5]);
    }

    #[test]
    fn delete() {
        let mut buffer = Buffer::<u32>::new();
        buffer.enter_slice(&[3, 1, 4, 1, 5]);
        assert_eq!(buffer.tokens.len(), 5);
        assert_eq!(buffer.cursor, 5);

        buffer.move_start();
        buffer.move_forward(2);
        assert_eq!(buffer.cursor, 2);

        buffer.delete(2);
        assert_eq!(buffer.cursor, 2);
        assert_eq!(buffer.tokens, &[3, 1, 5]);
    }

}
