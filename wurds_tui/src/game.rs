use crossterm::{
    cursor,
    style::{self, style, Color, Stylize},
    terminal, ExecutableCommand, QueueableCommand, Result,
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

    pub fn make_guess(&mut self, guess: &str) {
        self.game_state.make_guess(guess);
    }

    pub fn validate_input(&self, input: &str) -> bool {
        if input.len() != 5 {
            return false;
        }

        self.dictionary.contains(input)
    }

    pub fn run(&mut self) -> Result<()> {
        let mut input = String::new();

        while let GameState::InProgress = self.state() {
            self.stdout
                .execute(terminal::Clear(terminal::ClearType::All))?;
            self.draw_board()?;

            self.stdout
                .queue(cursor::MoveToNextLine(2))?
                .queue(cursor::SavePosition)?
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
            if !self.validate_input(input) {
                continue;
            }

            self.make_guess(&input.to_lowercase());

            self.stdout.execute(cursor::RestorePosition)?;
        }

        self.finish_game()
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
