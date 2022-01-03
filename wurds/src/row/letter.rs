use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct Letter {
    visibility: LetterVisibility,
    inner: char,
}

impl Letter {
    pub fn new(c: char, visibility: LetterVisibility) -> Self {
        Letter {
            visibility,
            inner: c,
        }
    }

    pub fn visibility(&self) -> LetterVisibility {
        self.visibility
    }

    pub(crate) fn set_visibility(&mut self, visibility: LetterVisibility) {
        self.visibility = visibility;
    }

    pub fn inner(&self) -> char {
        self.inner
    }
}

impl Display for Letter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LetterVisibility {
    Hidden,
    RevealedIncorrect,
    RevealedShifted,
    RevealedCorrect,
}
