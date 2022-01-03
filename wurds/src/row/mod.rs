mod letter;

use std::{collections::HashMap, ops::Index};

pub use self::letter::{Letter, LetterState};

#[derive(Debug, Clone, Copy)]
pub struct Row {
    letters: [Letter; 5],
    pub state: RowState,
}

#[derive(Debug, Clone, Copy)]
pub enum RowState {
    Hidden,
    Revealed,
}

impl Row {
    pub fn new(word: &str, state: RowState) -> Self {
        let letter_state = match state {
            RowState::Hidden => LetterState::Hidden,
            RowState::Revealed => LetterState::RevealedCorrect,
        };

        Row {
            letters: word
                .chars()
                .map(|c| Letter::new(c, letter_state))
                .collect::<Vec<_>>()
                .as_slice()
                .try_into()
                .unwrap(),
            state,
        }
    }

    pub fn new_guess(guess: &str, target: &str) -> Self {
        // We make a big assumption in this function that both `guess` and
        // `target` are of the same length!

        let mut char_counts = HashMap::new();

        for c in target.chars() {
            let entry = char_counts.entry(c).or_insert(0u32);
            *entry += 1;
        }

        let guess_chars = guess.as_bytes();
        let target_chars = target.as_bytes();

        let mut letters = guess
            .as_bytes()
            .iter()
            .map(|&c| Letter::new(c as char, LetterState::RevealedIncorrect))
            .collect::<Vec<_>>();

        // First, mark every character that is already in the correct spot.
        for (idx, &target_c) in target_chars.iter().enumerate() {
            if target_c == guess_chars[idx] {
                letters[idx].state = LetterState::RevealedCorrect;
                *char_counts.get_mut(&(target_c as char)).unwrap() -= 1;
            }
        }

        // Then, for every other character, check if it's shifted or incorrect.
        for (idx, &guess_c) in guess_chars.iter().enumerate() {
            // Skip the ones we've already marked as correct.
            if let LetterState::RevealedCorrect = letters[idx].state {
                continue;
            }

            letters[idx].state = match char_counts.get_mut(&(guess_c as char)) {
                None => LetterState::RevealedIncorrect,
                Some(count) => {
                    if *count < 1 {
                        LetterState::RevealedIncorrect
                    } else {
                        *count -= 1;
                        LetterState::RevealedShifted
                    }
                }
            }
        }

        let letters = letters.try_into().unwrap();

        Row {
            letters,
            state: RowState::Revealed,
        }
    }
}

impl Index<usize> for Row {
    type Output = Letter;

    fn index(&self, n: usize) -> &Self::Output {
        if n >= 5 {
            panic!(
                "index out of bounds: the len is {} but the index is {}",
                self.letters.len(),
                n
            );
        }

        &self.letters[n]
    }
}
