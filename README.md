## wurds

This workspace contains crates (projects) related to creating a game similar to [Wordle](https://www.powerlanguage.co.uk/wordle/).

Detailed documentation can be found in the respective crate's README.

### Library

The `wurds` crate is a general library that can be used to simulate the game. The library does not make any assumptions about the front-end: it can be used in a terminal-based application, as the backing model for a web API, or any other medium.

### Binaries

This workspace contains binaries that showcase applications that leverage the library, and are fully runnable.

- `wurds_tui` is an interactive text-based interface to the game
