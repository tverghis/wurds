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
        let mut char_counts = HashMap::new();

        for c in target.chars() {
            let entry = char_counts.entry(c).or_insert(0u32);
            *entry += 1;
        }

        let letters = guess
            .chars()
            .zip(target.chars())
            .map(|(g, t)| match char_counts.get_mut(&g) {
                None => Letter::new(g, LetterState::RevealedIncorrect),
                Some(count) => {
                    if *count < 1 {
                        return Letter::new(g, LetterState::RevealedIncorrect);
                    }

                    let state = if t == g {
                        LetterState::RevealedCorrect
                    } else {
                        LetterState::RevealedShifted
                    };

                    *count -= 1;

                    Letter::new(g, state)
                }
            })
            .collect::<Vec<_>>()
            .as_slice()
            .try_into()
            .unwrap();

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
