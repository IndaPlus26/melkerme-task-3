## melkerme Chess Library

### How to use

[1] In your `Cargo.toml` file, add the following:

```toml
[dependencies]
chess-library = { package = "melkerme-task-3", git = "https://github.com/IndaPlus26/melkerme-task-3" }
```

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

[5] Make a move:

```rust
let new_state = match game.make_move(String::from("E2"), String::from("E4")) {
    Some(state) => state,
    None => /* invalid move handling */,
};
game.set_game_state(new_state);
```

If the move is invalid, the function will return a `None` GameState option and
nothing in the game state will be changed.

[6] Get the game info:

```rust
let state = game.get_game_state(); // GameState::InProgress, GameState::Check, GameState::Checkmate, GameState::Stalemate, GameState::Draw
```

```rust
let board = game.get_board(); // A fen string representing the current board state.
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
