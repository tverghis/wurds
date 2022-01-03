use crate::{
    row::{Row, RowVisibility},
    MAX_GUESSES,
};

#[derive(Debug, Clone, Copy)]
pub enum GameResult {
    Win,
    Loss,
    InProgress,
}

pub struct GameState {
    word: String,
    rows: [Row; MAX_GUESSES],
    cur_guess: usize,
    result: GameResult,
}

impl GameState {
    pub fn new(word: String) -> Self {
        GameState {
            rows: [Row::new(&word, RowVisibility::Hidden); MAX_GUESSES],
            word,
            cur_guess: 1,
            result: GameResult::InProgress,
        }
    }

    pub fn guess_count(&self) -> usize {
        self.cur_guess
    }

    pub fn result(&self) -> GameResult {
        self.result
    }

    pub fn make_guess(&mut self, word: &str) {
        self.rows[self.cur_guess - 1] = Row::new_guess(word, &self.word);

        if word == self.word {
            self.result = GameResult::Win;
            return;
        }

        self.cur_guess += 1;

        if self.cur_guess > MAX_GUESSES {
            self.result = GameResult::Loss;
        }
    }

    pub fn row(&self, n: usize) -> &Row {
        &self.rows[n]
    }
}
