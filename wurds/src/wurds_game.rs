use crate::{
    row::letter::LetterVisibility,
    row::{Row, RowVisibility},
    MAX_GUESSES,
};

/// Possible states that the game can be in.
#[derive(Debug, Clone, Copy)]
pub enum GameState {
    /// User has won the game
    Win,
    /// User has lost the game
    Loss,
    /// Game is still in progress
    InProgress,
}

/// Models a `Wurds` game.
pub struct WurdsGame {
    word: String,
    rows: [Row; MAX_GUESSES],
    cur_guess: usize,
    state: GameState,
}

impl WurdsGame {
    /// Create a new game, with the given puzzle word.
    ///
    /// This function assumes that the puzzle word is valid.
    pub fn new(word: String) -> Self {
        WurdsGame {
            rows: [Row::new(&word, RowVisibility::Hidden); MAX_GUESSES],
            word,
            cur_guess: 1,
            state: GameState::InProgress,
        }
    }

    /// Returns the current attempt number.
    pub fn guess_count(&self) -> usize {
        self.cur_guess
    }

    /// Returns the [GameState] of the game.
    pub fn state(&self) -> GameState {
        self.state
    }

    /// Make a guess against the puzzle word.
    ///
    /// This function assumes that the guess is a valid word.
    pub fn make_guess(&mut self, word: &str) {
        self.rows[self.cur_guess - 1] = Row::new_guess(word, &self.word);

        if word == self.word {
            self.state = GameState::Win;
            return;
        }

        self.cur_guess += 1;

        if self.cur_guess > MAX_GUESSES {
            self.state = GameState::Loss;
        }
    }

    /// Returns the [Row] at index `n`.
    ///
    /// Panics if `n` is out of bounds.
    pub fn row(&self, n: usize) -> &Row {
        &self.rows[n]
    }
    pub fn rows(&self) -> &[Row; MAX_GUESSES] {
        &self.rows
    }
}
