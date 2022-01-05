mod letter;

use std::{collections::HashMap, ops::Index};

pub use self::letter::{Letter, LetterVisibility};

/// Models a (five-long) list of [Letter]s comprising a word that
/// the user has guessed.
#[derive(Debug, Clone, Copy)]
pub struct Row {
    letters: [Letter; 5],
    visibility: RowVisibility,
}

/// Possible visibility states of a [Row].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RowVisibility {
    /// Row contents are hidden.
    Hidden,
    /// Row contents are revealed.
    Revealed,
}

impl Row {
    /// Create a new Row, containing the given word. The Row may be created as
    /// hidden or revealed.
    pub(crate) fn new(word: &str, visibility: RowVisibility) -> Self {
        let letter_state = match visibility {
            RowVisibility::Hidden => LetterVisibility::Hidden,
            RowVisibility::Revealed => LetterVisibility::RevealedCorrect,
        };

        Row {
            letters: word
                .chars()
                .map(|c| Letter::new(c, letter_state))
                .collect::<Vec<_>>()
                .as_slice()
                .try_into()
                .unwrap(),
            visibility,
        }
    }

    /// Returns whether the Row's contents are hidden or revealed.
    pub fn is_visible(&self) -> bool {
        self.visibility == RowVisibility::Revealed
    }

    /// Create a new row, `guess`ing against the `target` word.
    ///
    /// The [Letter]s in this row are marked as being correct or incorrect.
    /// The Row itself is created as revealed.
    pub(crate) fn new_guess(guess: &str, target: &str) -> Self {
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
            .map(|&c| Letter::new(c as char, LetterVisibility::RevealedIncorrect))
            .collect::<Vec<_>>();

        // First, mark every character that is already in the correct spot.
        for (idx, &target_c) in target_chars.iter().enumerate() {
            if target_c == guess_chars[idx] {
                letters[idx].set_visibility(LetterVisibility::RevealedCorrect);
                *char_counts.get_mut(&(target_c as char)).unwrap() -= 1;
            }
        }

        // Then, for every other character, check if it's shifted or incorrect.
        for (idx, &guess_c) in guess_chars.iter().enumerate() {
            let letter = &mut letters[idx];

            // Skip the ones we've already marked as correct.
            if let LetterVisibility::RevealedCorrect = letter.visibility() {
                continue;
            }

            let vis = match char_counts.get_mut(&(guess_c as char)) {
                None => LetterVisibility::RevealedIncorrect,
                Some(count) => {
                    if *count < 1 {
                        LetterVisibility::RevealedIncorrect
                    } else {
                        *count -= 1;
                        LetterVisibility::RevealedShifted
                    }
                }
            };

            letter.set_visibility(vis);
        }

        let letters = letters.try_into().unwrap();

        Row {
            letters,
            visibility: RowVisibility::Revealed,
        }
    }

    pub fn iter(&self) -> RowIterator {
        RowIterator {
            row: self,
            index: 0,
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

pub struct RowIterator<'a> {
    row: &'a Row,
    index: usize,
}

impl<'a> Iterator for RowIterator<'a> {
    type Item = Letter;

    fn next(&mut self) -> Option<Letter> {
        if self.index < 5 {
            let l = self.row[self.index];
            self.index += 1;
            Some(l)
        } else {
            None
        }
    }
}
