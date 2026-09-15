## melkerme Chess Library

### How to use

[1] In your `Cargo.toml` file, add the following:

```toml
[dependencies]
chess-library = { package = "melkerme-task-3", git = "ssh://git@github.com/IndaPlus26/melkerme-task-3.git" }
```

`If you have issues, make sure env.CARGO_NET_GIT_FETCH_WITH_CLI is set to true.`
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
Alternatively, you can create a game from a FEN string:

```rust
let mut game = Game::new_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
```

[4] Get the possible moves for a piece:

```rust
let moves = game.get_possible_moves(String::from("E2"));
```

If there are no possible moves from the given square, the function will return a
`None` option. Else it will return a vector of Strings representing the possible
`to` squares. Eg. `["E4", "E3"]`.

[5] Make a move: Here's an example of how to make a move and set the game state:

```rust
let result = make_move(from, to);
if let Some(game_state) = result {
    set_game_state(game_state);
} else {
    /* Here you can handle situations where the function returns None, for example if your try to play an invalid move. */
}
```

If the move is invalid, the function will return a `None` GameState option and
nothing in the game state will be changed.

[6] Get the game info:

```rust
let state = game.get_game_state();
```

```rust
let board = game.to_fen(); // A fen string representing the current board state.
```

```rust
let turn_color = game.get_turn_color(); // Color::White or Color::Black
```

```rust
let promotion_piece = game.get_promotion_piece(); // Piece::Queen, Piece::Rook, Piece::Bishop, Piece::Knight
```

```rust
let halfmove = game.get_halfmove(); // The halfmove clock as a u32.
```

```rust
let fullmove = game.get_fullmove(); // The fullmove clock as a u32.
```
