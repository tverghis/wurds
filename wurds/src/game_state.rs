use crate::{
    row::{Row, RowVisibility},
    MAX_GUESSES,
};

#[derive(Debug, Clone, Copy)]
pub enum GameState {
    Win,
    Loss,
    InProgress,
}

pub struct WurdsGame {
    word: String,
    rows: [Row; MAX_GUESSES],
    cur_guess: usize,
    state: GameState,
}

impl WurdsGame {
    pub fn new(word: String) -> Self {
        WurdsGame {
            rows: [Row::new(&word, RowVisibility::Hidden); MAX_GUESSES],
            word,
            cur_guess: 1,
            state: GameState::InProgress,
        }
    }

    pub fn guess_count(&self) -> usize {
        self.cur_guess
    }

    pub fn state(&self) -> GameState {
        self.state
    }

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

    pub fn row(&self, n: usize) -> &Row {
        &self.rows[n]
    }
}
