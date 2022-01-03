use crate::{
    row::{Row, RowState},
    MAX_GUESSES,
};

#[derive(Debug, Clone, Copy)]
pub enum GameResult {
    Win,
    Loss,
    InProgress,
}

pub struct GameState {
    pub word: String,
    pub rows: [Row; MAX_GUESSES],
    pub cur_guess: usize,
    pub result: GameResult,
}

impl GameState {
    pub fn new(word: String) -> Self {
        GameState {
            rows: [Row::new(&word, RowState::Hidden); MAX_GUESSES],
            word,
            cur_guess: 1,
            result: GameResult::InProgress,
        }
    }

    pub fn guess(&mut self, word: &str) {
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
}
