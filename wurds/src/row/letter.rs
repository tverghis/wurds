use std::fmt::Display;

/// A simple wrapped around a letter in a guessed word.
///
/// In addition to containing a character, a Letter also
/// contains its own [LetterVisibility].
#[derive(Debug, Clone, Copy)]
pub struct Letter {
    visibility: LetterVisibility,
    inner: char,
}

impl Letter {
    /// Create a new Letter with the specified visibility.
    pub fn new(c: char, visibility: LetterVisibility) -> Self {
        Letter {
            visibility,
            inner: c,
        }
    }

    /// Returns the visibility of the Letter.
    pub fn visibility(&self) -> LetterVisibility {
        self.visibility
    }

    /// Set the visibility of the Letter.
    pub(crate) fn set_visibility(&mut self, visibility: LetterVisibility) {
        self.visibility = visibility;
    }

    /// Returns the wrapped raw character.
    pub fn inner(&self) -> char {
        self.inner
    }
}

impl Display for Letter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

/// Represents the various states that a Letter can be in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LetterVisibility {
    /// The letter is hidden entirely
    Hidden,
    /// The letter is revealed, but it is not present in the puzzle word.
    RevealedIncorrect,
    /// The letter is revealed, but it is present in the incorrect location.
    RevealedShifted,
    /// The letter is revealed, and it is present in the correct location.
    RevealedCorrect,
}
