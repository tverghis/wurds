use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct Letter {
    pub state: LetterState,
    pub inner: char,
}

impl Letter {
    pub fn new(c: char, state: LetterState) -> Self {
        Letter { state, inner: c }
    }
}

impl Display for Letter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum LetterState {
    Hidden,
    RevealedIncorrect,
    RevealedShifted,
    RevealedCorrect,
}
