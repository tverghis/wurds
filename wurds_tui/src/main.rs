mod game;

use std::{
    collections::HashSet,
    io::{Stdout, Write},
};

use crossterm::{
    cursor,
    style::{self, style, Color, Stylize},
    terminal, ExecutableCommand, QueueableCommand, Result,
};

use rand::Rng;

use wurds::{
    game_state::{GameResult, GameState},
    row::{LetterState, RowState},
    MAX_GUESSES, WORD_SIZE,
};

const DICTIONARY: &str = include_str!("../dict.txt");

fn main() -> Result<()> {
    let mut stdout = std::io::stdout();
    let stdin = std::io::stdin();
    let mut input = String::new();

    let dictionary = DICTIONARY.lines().collect::<Vec<_>>();
    let mut dictionary_set = HashSet::<&str>::with_capacity(dictionary.len());
    dictionary_set.extend(dictionary.iter());

    let mut rng = rand::thread_rng();
    let word = dictionary[rng.gen_range(0..dictionary.len())].into();

    let mut game = GameState::new(word);

    while let GameResult::InProgress = game.result {
        stdout.execute(terminal::Clear(terminal::ClearType::All))?;
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
        if !input_is_valid(input, &dictionary_set) {
            continue;
        }

        game.guess(&input.to_lowercase());

        stdout.execute(cursor::RestorePosition)?;
    }

    finish_game(&game, &mut stdout)
}

fn input_is_valid(input: &str, dictionary: &HashSet<&str>) -> bool {
    if input.len() != WORD_SIZE {
        return false;
    }

    dictionary.contains(input)
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

            let letter = game.rows[row][col];
            let letter_repr = match letter.state {
                LetterState::Hidden => '\u{25A1}',
                _ => letter.inner,
            };

            stdout.queue(style::PrintStyledContent(
                style(letter_repr).with(letter_color),
            ))?;
        }
    }

    stdout.flush()?;

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
        _ => style(format!("You lost! The word was `{}`.", game.word)).with(Color::Red),
    };

    stdout
        .queue(cursor::MoveToNextLine(2))?
        .queue(terminal::Clear(terminal::ClearType::FromCursorDown))?
        .queue(style::PrintStyledContent(message))?
        .queue(cursor::MoveToNextLine(1))?;

    Ok(())
}
