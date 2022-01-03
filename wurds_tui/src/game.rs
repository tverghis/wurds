use crossterm::{
    cursor,
    style::{self, style, Color, Stylize},
    terminal, QueueableCommand, Result,
};
use rand::Rng;
use std::{
    collections::HashSet,
    io::{Stdin, Stdout, Write},
};
use wurds::{
    game_state::{GameState, WurdGame},
    row::LetterVisibility,
    MAX_GUESSES, WORD_SIZE,
};

pub struct Game<'a> {
    dictionary: HashSet<&'a str>,
    target_word: &'a str,
    game_state: WurdGame,
    stdin: Stdin,
    stdout: Stdout,
}

impl<'a> Game<'a> {
    pub fn new(dict: &[&'a str]) -> Self {
        let mut rng = rand::thread_rng();
        let num_words = dict.len();
        let target_word = dict[rng.gen_range(0..num_words)];

        let mut dictionary = HashSet::with_capacity(num_words);
        dictionary.extend(dict.iter());

        let game_state = WurdGame::new(target_word.into());

        let stdin = std::io::stdin();
        let stdout = std::io::stdout();

        Game {
            dictionary,
            target_word,
            game_state,
            stdin,
            stdout,
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
        if input.len() != 5 {
            return InputValidationResult::IncorrectLength;
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InputValidationResult {
    Valid,
    IncorrectLength,
    Unrecognized,
}
