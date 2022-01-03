use std::fmt::Display;

use crate::BLOCK_CHAR;

#[derive(Debug, Clone, Copy)]
pub struct Letter {
    pub state: LetterState,
    inner: char,
}

impl Letter {
    pub fn new(c: char, state: LetterState) -> Self {
        Letter { state, inner: c }
    }
}

impl Display for Letter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.state {
            LetterState::Hidden => write!(f, "{}", BLOCK_CHAR),
            _ => write!(f, "{}", self.inner),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum LetterState {
    Hidden,
    RevealedIncorrect,
    RevealedShifted,
    RevealedCorrect,
}
