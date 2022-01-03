mod letter;

use std::{collections::HashSet, ops::Index};

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
        let target_chars = target.chars().collect::<HashSet<_>>();

        let letters = guess
            .chars()
            .zip(target.chars())
            .map(|(g, t)| {
                if g == t {
                    return Letter::new(g, LetterState::RevealedCorrect);
                }

                /*
                 * FIXME:
                 * If the `target` character has been previously revealed in the
                 * correct position, we will incorrectly mark this occurence of
                 * `target` as `RevealedShifted`.
                 * For example, say that the word being guessed is "hello".
                 * If the user enters a guess of `heels`, then the first `e`
                 * will be marked as `RevealedCorrect`, but the second `e` will
                 * be marked `RevealedShifted`. The second `e` should be marked
                 * `RevealedIncorrect` instead, so as to not mislead the player
                 * into thinking the word contains a second `e`.
                 */
                if target_chars.contains(&g) {
                    return Letter::new(g, LetterState::RevealedShifted);
                }

                Letter::new(g, LetterState::RevealedIncorrect)
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
