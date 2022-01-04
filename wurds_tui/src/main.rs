mod game;
mod game_opts;

use clap::Parser;
use game::Game;
use game_opts::{GameArgs, GameOpts};

const DICTIONARY: &str = include_str!("../dict.txt");

fn main() -> anyhow::Result<()> {
    let args = GameArgs::parse();

    Game::new(GameOpts::from_args(args)?).run()
}
