## melkerme Chess Library

### How to use

[1] In your `Cargo.toml` file, add the following:

```toml
[dependencies]
chess-library = { git = "https://github.com/IndaPlus26/melkerme-task-3" }
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
let moves = game.get_possible_moves(String::from("e2"));
```

If there are no possible moves from the given square, the function will return a
`None` option. Else it will return a vector of Strings representing the possible
moves. Eg. `["e2e4", "e2e3"]`.

[5] Make a move:

```rust
let new_state = match game.make_move(String::from("e2"), String::from("e4")) {
    Some(state) => state,
    None => /* invalid move handling */,
};
game.set_game_state(new_state);
```

If the move is invalid, the function will return a `None` GameState option and
the game state will not be changed.

[6] Get the game info:

```rust
let state = game.get_game_state(); // GameState::InProgress, GameState::Check, GameState::Checkmate, GameState::Stalemate, GameState::Draw
```

```rust
let board = game.get_board(); // A 2D array of char representing the board. The characters are:
                              - 'P' for white pawn
                              - 'R' for white rook
                              - 'N' for white knight
                              - 'B' for white bishop
                              - 'Q' for white queen
                              - 'K' for white king
                              - 'p' for black pawn
                              - 'r' for black rook
                              - 'n' for black knight
                              - 'b' for black bishop
                              - 'q' for black queen
                              - 'k' for black king
                              - '.' for empty square
```

```rust
let turn_color = game.get_turn_color(); // "white" or "black"
```

```rust
let promotion_piece = game.get_promotion_piece(); // "queen", "rook", "bishop", "knight"
```

```rust
let halfmove = game.get_halfmove(); // The halfmove clock as a u32.
```

```rust
let fullmove = game.get_fullmove(); // The fullmove clock as a u32.
```
