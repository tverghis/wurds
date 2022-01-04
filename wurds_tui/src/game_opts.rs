use std::{fs::File, io::Read};

use anyhow::{bail, Result};
use clap::Parser;
use wurds::WORD_SIZE;

use crate::DICTIONARY;

#[derive(Parser)]
#[clap(about, version, author)]
pub struct GameArgs {
    /// Force a word to be used as the puzzle word
    #[clap(long = "forced-word", short = 'w')]
    forced_word: Option<String>,

    /// Use a specified dictionary instead of the default one
    #[clap(long, short)]
    dictionary: Option<String>,

    /// Don't require that inputs are recognized by the dictionary
    #[clap(long = "free-input", short = 'i')]
    free_input: bool,
}

pub struct GameOpts {
    pub dictionary: Vec<String>,
    pub free_input: bool,
    pub forced_word: Option<String>,
}

impl GameOpts {
    pub fn from_args(args: GameArgs) -> Result<Self> {
        let mut dictionary = match args.dictionary {
            None => DICTIONARY.lines().map(Into::into).collect::<Vec<_>>(),
            Some(path) => load_dict_from_file(&path)?,
        };

        let forced_word = match args.forced_word {
            None => None,
            Some(word) => Some(validate_forced_word(word)?),
        };

        // If the user wants to force a word, make sure it exists in the dictionary
        if let Some(forced) = forced_word.as_ref().cloned() {
            dictionary.push(forced);
        }

        Ok(GameOpts {
            dictionary,
            free_input: args.free_input,
            forced_word,
        })
    }
}

fn load_dict_from_file(file_path: &str) -> Result<Vec<String>> {
    let mut file = File::open(file_path)?;
    let mut buf = String::new();

    file.read_to_string(&mut buf)?;

    Ok(buf
        .lines()
        .map(str::trim)
        .filter(|line| line.len() == WORD_SIZE)
        .map(Into::into)
        .collect())
}

fn validate_forced_word(forced_word: String) -> Result<String> {
    if forced_word.len() != WORD_SIZE {
        bail!("forced word must be of length 5");
    }

    Ok(forced_word)
}
