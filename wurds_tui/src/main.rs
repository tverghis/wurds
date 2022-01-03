mod game;

use game::Game;

const DICTIONARY: &str = include_str!("../dict.txt");

fn main() {
    let dictionary = DICTIONARY.lines().collect::<Vec<_>>();

    Game::new(&dictionary).run().unwrap();
}
