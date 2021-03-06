use crate::location::Location;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn string_stream() {
        let string = String::from("What a nice content,\nall in a single stream!");
        let origin = String::from("somewhere");
        let stream = StringStream::new(origin.clone(), string.clone());
        assert_eq!(stream.borrow(), string.as_str());
        for &i in [0, 3, 5, 17, string.len(), string.len() + 2].iter() {
            let (chr, loc) = stream.get_at(i).unwrap();
            if i < string.len() {
                if let Char::Char(chr) = chr {
                    assert_eq!(chr, string.chars().nth(i).unwrap());
                } else {
                    panic!(format!(
                        "There is a char at position {} of {}, and yet it was not in the stream",
                        i, &string
                    ));
                }
                let location = Location::from_stream_pos(origin.clone(), &string[..], i, i);
                assert_eq!(location, loc);
            } else {
                if let Char::Char(_) = chr {
                    panic!(format!(
                        "There is no char at position {} of {}, and yet there is one in the stream",
                        i, &string
                    ));
                }
                let location = Location::from_stream_pos(
                    origin.clone(),
                    &string[..],
                    string.len(),
                    string.len(),
                );
                assert_eq!(location, loc);
            }
        }
    }
}

/// The type of data returned by the stream.
/// Is a tuple of two elements, the element
/// at a given index and its location.
pub type StreamObject<T> = (T, Location);

/// # Summary
///
/// A stream-like object.
///
/// # Methods
///
/// `get_at`: get an object at a given index.
/// `pos`: return the current position.
/// `set_pos`: set the current position.
/// `pos_pp`: increment the current position by one.
/// `pos_inc`: increment the current position by a given (positive) ammount.
/// `get`: get the object at the current position.
pub trait Stream<'a> {
    type Output: 'a;
    /// Get the object at the given position.
    fn get_at(&'a self, pos: usize) -> Option<StreamObject<Self::Output>>;
    /// Return the current position.
    fn pos(&self) -> usize;
    /// Set the current position.
    fn set_pos(&mut self, pos: usize);
    /// Increase the current position by one.
    fn pos_pp(&mut self) {
        self.set_pos(self.pos() + 1);
    }
    /// Increase the current position by a given (positive) ammount;
    fn pos_inc(&mut self, off: usize) {
        self.set_pos(self.pos() + off);
    }
    /// Get the object at the current position.
    fn get(&'a self) -> Option<StreamObject<Self::Output>> {
        self.get_at(self.pos())
    }
}

/// # Summary
///
/// A character, or `EOF`.
///
/// # Variants
///
/// `Char(char)`: a character.
/// `EOF`: End Of File.
pub enum Char {
    Char(char),
    EOF,
}

/// # Summary
///
/// A stream based on a string, considered as a file-like object.
/// Thus, a `StringStream` object requires an `origin`.
///
/// # Methods
/// `new`: build a `StringStream`.
/// `continues`: returns if the substring that starts at the current position matches the given one.
/// `borrow`: borrows (read-only) the stream as a string slice
/// `len`: the size of the stream
/// `is_empty`: whether the stream is empty
pub struct StringStream {
    origin: String,
    stream: Vec<(char, Location)>,
    pos: usize,
    length: usize,
    end_pos: Location,
}

impl StringStream {
    /// Build a new `StringStream`, based on its `origin` and on a given `string`.
    pub fn new(origin: String, string: String) -> Self {
        let mut current_char = 0;
        let mut current_line = 0;
        let mut stream = Vec::new();
        for chr in string.chars() {
            let pos = (current_line, current_char);
            stream.push((chr, Location::new(origin.clone(), pos, pos)));
            if chr == '\n' {
                current_line += 1;
                current_char = 0;
            } else {
                current_char += 1;
            }
        }
        Self {
            origin: origin.clone(),
            stream,
            pos: 0,
            length: string.len(),
            end_pos: Location::new(
                origin,
                (current_line, current_char),
                (current_line, current_char),
            ),
        }
    }

    /// Return a boolean corresponding to whether the substring of
    /// the `StringStream` that starts at the current position matches
    /// the given string.
    pub fn continues(&self, keyword: &str) -> bool {
        let size = keyword.len();
        if self.pos + size > self.length {
            return false;
        }
        let mut result = true;
        let mut chars = keyword.chars();
        for i in 0..size {
            if self.stream[self.pos + i].0 != chars.next().unwrap() {
                result = false;
                break;
            }
        }
        result
    }

    /// Return a string slice corresponding to the
    /// underlying string.
    pub fn borrow(&self) -> String {
        self.stream.iter().map(|(chr, _)| chr).collect()
    }

    pub fn origin(&self) -> &str {
        &self.origin[..]
    }

    /// Return the length of the stream.
    pub fn len(&self) -> usize {
        self.stream.len()
    }

    /// Return is the stream is empty.
    pub fn is_empty(&self) -> bool {
        self.stream.is_empty()
    }
}

impl Stream<'_> for StringStream {
    type Output = Char;
    fn get_at(&self, pos: usize) -> Option<StreamObject<Self::Output>> {
        Some(match self.stream.get(pos) {
            Some((chr, pos)) => (Char::Char(*chr), pos.clone()),
            None => (Char::EOF, self.end_pos.clone()),
        })
    }
    fn set_pos(&mut self, pos: usize) {
        self.pos = pos;
    }
    fn pos(&self) -> usize {
        self.pos
    }
}

impl std::fmt::Debug for StringStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.borrow().fmt(f)
    }
}
