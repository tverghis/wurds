mod game_state;
mod row;

use std::io::{Stdout, Write};

use crossterm::{
    cursor,
    style::{self, style, Color, Stylize},
    terminal, ExecutableCommand, QueueableCommand, Result,
};
use game_state::{GameResult, GameState};
use row::{LetterState, RowState};

const BLOCK_CHAR: char = '\u{25A1}'; // "WHITE SQUARE"
const MAX_GUESSES: usize = 5;
const WORD_SIZE: usize = 5;

fn main() -> Result<()> {
    let mut stdout = std::io::stdout();
    let stdin = std::io::stdin();
    let mut input = String::new();

    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    let mut game = GameState::new(String::from("hello"));

    while let GameResult::InProgress = game.result {
        draw_board(&game, &mut stdout)?;

        stdout
            .queue(cursor::MoveToNextLine(2))?
            .queue(cursor::SavePosition)?
            .queue(style::PrintStyledContent(
                format!("Guess ({}/{}): ", game.cur_guess, MAX_GUESSES).green(),
            ))?
            .queue(terminal::Clear(terminal::ClearType::UntilNewLine))?;

        stdout.flush()?;

        input.clear();

        stdin.read_line(&mut input)?;

        let input = input.trim();
        if input.len() != WORD_SIZE {
            continue;
        }

        game.guess(input);

        stdout.execute(cursor::RestorePosition)?;
    }

    finish_game(&game, &mut stdout)
}

fn draw_board(game: &GameState, stdout: &mut Stdout) -> Result<()> {
    for row in 0..MAX_GUESSES {
        for col in 0..WORD_SIZE {
            stdout.queue(cursor::MoveTo(1 + col as u16 * 3, row as u16))?;

            let letter_color = match game.rows[row].state {
                RowState::Hidden => Color::DarkGrey,
                RowState::Revealed => match game.rows[row][col].state {
                    LetterState::RevealedShifted => Color::Yellow,
                    LetterState::RevealedCorrect => Color::Green,
                    _ => Color::DarkGrey,
                },
            };

            stdout.queue(style::PrintStyledContent(
                style(game.rows[row][col]).with(letter_color),
            ))?;
        }
    }

    Ok(())
}

fn finish_game(game: &GameState, stdout: &mut Stdout) -> Result<()> {
    draw_board(game, stdout)?;

    let message = match game.result {
        GameResult::Win => style(format!(
            "Congratulations, you won! ({}/{})",
            game.cur_guess, MAX_GUESSES
        ))
        .with(Color::Green),
        _ => style(String::from("You lost!")).with(Color::Red),
    };

    stdout
        .queue(cursor::MoveToNextLine(2))?
        .queue(terminal::Clear(terminal::ClearType::FromCursorDown))?
        .queue(style::PrintStyledContent(message))?
        .queue(cursor::MoveToNextLine(1))?;

    Ok(())
}
