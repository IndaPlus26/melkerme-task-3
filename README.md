## melkerme-task-3

### Setup

[1] In your `Cargo.toml` file, add the following:

```toml
[dependencies]
chess-library = { package = "melkerme-task-3", git = "ssh://git@github.com/IndaPlus26/melkerme-task-3.git" }
```
`If you have issues, make sure env.CARGO_NET_GIT_FETCH_WITH_CLI is set to true.`

You can also clone the repo and use:
```toml
[dependencies]
chess_library = { package = "melkerme-task-3", path = "path/to/repo" }
```
You can also check out the `bot` branch if you want a bot to play against.

[2] In your `main.rs` file, add the following:

```rust
use chess_library::Game;
```

[3] Create a new game:

```rust
let mut game = Game::new();
```

This will create a new game with the default chess starting position.

### Docs
Full documentation can be found [here](index.html)

### Useful info
* All possible chess moves are available: castling, promotion to all 4 pieces and en passant included. (Open an issue if you find a bug)
* You are in control of the game state, it will not set itself. You will get the new game state from the make_move function, I recommend setting it when a move is made -> handle the new game state: play a sound etc -> set it back to InProgress before the next move is made.
