use crate::game_opts::GameOpts;
use anyhow::Result;
use crossterm::{
    cursor,
    style::{self, style, Color, Stylize},
    terminal, QueueableCommand,
};
use rand::Rng;
use std::{
    collections::{HashMap, HashSet},
    io::{Stdin, Stdout, Write},
};
use wurds::{
    row::LetterVisibility,
    wurds_game::{GameState, WurdsGame},
    MAX_GUESSES, WORD_SIZE,
};

pub struct Game {
    dictionary: HashSet<String>,
    target_word: String,
    game_state: WurdsGame,
    stdin: Stdin,
    stdout: Stdout,
    opts: GameOpts,
}

impl Game {
    pub fn new(opts: GameOpts) -> Self {
        let dict = &opts.dictionary;
        let num_words = dict.len();

        let target_word = opts.forced_word.as_ref().cloned().unwrap_or_else(|| {
            let mut rng = rand::thread_rng();
            dict[rng.gen_range(0..num_words)].clone()
        });

        let dictionary = dict.iter().map(Into::into).collect();

        let game_state = WurdsGame::new(target_word.clone());

        let stdin = std::io::stdin();
        let stdout = std::io::stdout();

        Game {
            dictionary,
            target_word,
            game_state,
            stdin,
            stdout,
            opts,
        }
    }

    fn state(&self) -> GameState {
        self.game_state.state()
    }

    pub fn run(&mut self) -> Result<()> {
        let mut input = String::new();
        let mut status_line: Option<&'static str> = None;

        self.stdout
            .queue(terminal::Clear(terminal::ClearType::All))?
            .queue(cursor::SavePosition)?;

        self.stdout.flush()?;

        while let GameState::InProgress = self.state() {
            self.draw_board()?;

            self.stdout.queue(cursor::MoveToNextLine(2))?;

            self.stdout
                .queue(terminal::Clear(terminal::ClearType::CurrentLine))?;
            if let Some(status) = status_line {
                self.stdout
                    .queue(style::PrintStyledContent(status.dark_grey()))?;
            }
            self.stdout.queue(cursor::MoveToNextLine(1))?;

            self.draw_letterstate()?;

            self.stdout
                .queue(style::PrintStyledContent(
                    format!(
                        "Guess ({}/{}): ",
                        self.game_state.guess_count(),
                        MAX_GUESSES
                    )
                    .green(),
                ))?
                .queue(terminal::Clear(terminal::ClearType::UntilNewLine))?;

            self.stdout.flush()?;

            input.clear();

            self.stdin.read_line(&mut input)?;

            let input = input.trim();

            let validated = self.validate_input(input);

            status_line = match validated {
                InputValidationResult::Valid => None,
                InputValidationResult::IncorrectLength => Some("Try guessing a five-letter word!"),
                InputValidationResult::Unrecognized => Some("Didn't recognize that word!"),
            };

            if let InputValidationResult::Valid = validated {
                self.game_state.make_guess(&input.to_lowercase());
            }
        }

        self.finish_game()?;
        self.stdout.flush()?;

        Ok(())
    }

    fn validate_input(&self, input: &str) -> InputValidationResult {
        if input.len() != WORD_SIZE {
            return InputValidationResult::IncorrectLength;
        }

        if self.opts.free_input {
            return InputValidationResult::Valid;
        }

        if self.dictionary.contains(input) {
            InputValidationResult::Valid
        } else {
            InputValidationResult::Unrecognized
        }
    }

    fn draw_board(&mut self) -> Result<()> {
        for row in 0..MAX_GUESSES {
            for col in 0..WORD_SIZE {
                self.stdout
                    .queue(cursor::MoveTo(1 + col as u16 * 3, row as u16))?;

                let row = self.game_state.row(row);

                let letter_color = if row.is_visible() {
                    match row[col].visibility() {
                        LetterVisibility::RevealedShifted => Color::Yellow,
                        LetterVisibility::RevealedCorrect => Color::Green,
                        _ => Color::DarkGrey,
                    }
                } else {
                    Color::DarkGrey
                };

                let letter = row[col];
                let letter_repr = match letter.visibility() {
                    LetterVisibility::Hidden => '\u{25A1}',
                    _ => letter.inner(),
                };

                self.stdout.queue(style::PrintStyledContent(
                    style(letter_repr).with(letter_color),
                ))?;
            }
        }

        self.stdout.flush()?;

        Ok(())
    }

    fn finish_game(&mut self) -> Result<()> {
        self.stdout.queue(cursor::RestorePosition)?;
        self.draw_board()?;

        let message = match self.state() {
            GameState::Win => style(format!(
                "Congratulations, you won! ({}/{})",
                self.game_state.guess_count(),
                MAX_GUESSES
            ))
            .with(Color::Green),
            _ => style(format!("You lost! The word was `{}`.", self.target_word)).with(Color::Red),
        };

        self.stdout
            .queue(cursor::MoveToNextLine(2))?
            .queue(terminal::Clear(terminal::ClearType::FromCursorDown))?
            .queue(style::PrintStyledContent(message))?
            .queue(cursor::MoveToNextLine(1))?;

        Ok(())
    }

    fn draw_letter(&mut self, letter: char, visibility: &LetterVisibility) -> Result<()> {
        let color = match visibility {
            LetterVisibility::Hidden => Color::White,
            LetterVisibility::RevealedIncorrect => Color::DarkGrey,
            LetterVisibility::RevealedShifted => Color::Yellow,
            LetterVisibility::RevealedCorrect => Color::Green,
        };

        self.stdout.queue(style::PrintStyledContent(
            style(format!("{} ", letter)).with(color),
        ))?;
        Ok(())
    }
    fn draw_letterstate(&mut self) -> Result<()> {
        let letters_guessed = self
            .game_state
            .rows()
            .iter()
            .filter(|row| row.is_visible())
            .flat_map(|row| row.iter().collect::<Vec<_>>())
            .fold(HashMap::new(), |mut acc, letter| {
                acc.entry(letter.inner())
                    .and_modify(|visibility| {
                        *visibility = std::cmp::max(*visibility, letter.visibility())
                    })
                    .or_insert_with(|| letter.visibility());
                acc
            });

        for (start, end) in [(b'a', b'm'), (b'n', b'z')] {
            self.stdout
                .queue(terminal::Clear(terminal::ClearType::CurrentLine))?;
            for c in (start..=end).map(char::from) {
                self.draw_letter(
                    c,
                    letters_guessed.get(&c).unwrap_or(&LetterVisibility::Hidden),
                )?;
            }
            self.stdout.queue(cursor::MoveToNextLine(1))?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InputValidationResult {
    Valid,
    IncorrectLength,
    Unrecognized,
}
