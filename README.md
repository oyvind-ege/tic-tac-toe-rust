# My Tic-Tac-Toe Game in Rust

A command-line tic-tac-toe game I built while learning Rust. You can play against an AI opponent that uses the minimax algorithm to make optimal moves.

<img width="603" alt="Screenshot 2025-06-06 at 10 53 11" src="https://github.com/user-attachments/assets/76c5007f-447d-4c82-8046-445bec9c8322" />

## On the use of LLMs
>[!NOTE]
>This project is almost exclusively written by me, and not by AI. 
> I have used Claude Sonnet 4 via [Aider](https://aider.chat/) for advice and feedback during development, and it sometimes made commits which I think I have largely reverted.

## Features

- [x] **Play against the computer** - the AI uses the [Minimax algorithm](https://www.neverstopbuilding.com/blog/minimax) to make optimal moves. If you lose against this AI, you've made a suboptimal move!
- [x] **3x3 game board** - Supports and is tested with a 3x3 game board, but with some extensibility for a larger game board.
- [X] **Help and exit functionality in game loop** - Luckily, you can actually quit the game, and get basic help on how to not suck.


### Larger things I work on right now

- [ ] **Event-based / Pub-sub architecture** - The end users don't care, but would be cool to do and will set me up for multiplayer and graphics down the line.

### Soon?

- [ ] **Optimizing AI** - It works, but for larger boards it can be a problem. Here are some optimizations I can do:
    - [ ] *Alpha/Beta pruning* - An optimization that discards large parts of the AI state tree
    - [ ] *Depth limitation* - Currently, the AI minimax algorithm will traverse the entire tree until a terminal state has been reached, regardless of how "deep" it is. For a 3x3 board, this is fine, but if I decide to truly support larger boards, this needs to change (along with alpha/beta pruning)
- [ ] **Multiplayer** - High on my list of wants. It will help me learn networking!
- [ ] **Graphics** - Also high on my list. Either with [SDL2](https://github.com/Rust-SDL2/rust-sdl2), or with a framework like [Tauri](https://v2.tauri.app/) that would allow me to write frontend in React or Svelte.
- [ ] **4x4 or 5x5 boards** - May or may not do this. I don't think it will be difficult to achieve.

## Architecture

### Core components

```
src/
├── main.rs              # Entry point and game initialization
├── gamestate/           # Game state structure and main game loop logic
├── board.rs             # Board representation and game logic
├── player/              # Player management and types
│   ├── base_player.rs    # Player definitions and controller traits
│   └── playerlist.rs    # Player collection and iteration
├── controller.rs        # Input handling and validation traits
└── ai/                  # AI implementation
    ├── mod.rs          # AI strategy enumeration
    └── minimax.rs      # Minimax algorithm implementation
```

### How I structured things

#### **Using Traits for Flexibility**
Traits is the Rust equivalent of interfaces. I have a couple in my code, mainly for the learning experience:

- `PlayerController` trait is a layer of indirection between the GameState core structure(class) and agents that can interact with the game
- `InputController` trait, which provides a layer of indirection which may later support mouse input and whatnot

This might very well be overengineered, but I don't care!

#### **Enums**
I love enums, because they ensure that 'something' is of a limited set of variants, at compile-time. Here are some of mine:

```rust
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum CellState {
    Empty,
    Player(u8),
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Diagonal {
    Major,
    Minor,
}

#[derive(Debug, Clone)]
pub enum BoardError {
    CellOccupied(usize),
    OutOfBounds(usize),
}
```

I initially used basic unsigned 8-bit integers (u8) as the Cell representation, but effectively decided to use a CellState enum so that I could enforce either Empty or Player-occupied cells.
Plus, using enums makes the code more self-explanatory!

#### **Error Handling**

I am not entirely happy yet. Working on making error handling more standardized and idiomatic. But so far I have:

- Custom error types (`BoardError`, `InputError`)
- Tries to give useful feedback when things go wrong

#### **Input validation - for AI and Humans**
I kept the validation flow straightforward. Input validation happens primarily on the human player side when parsing user input.
I assume AI never suggests an out-of-bounds move, or a move on a non-empty cell, because of the implementation.

```rust
// Human player input flow:
get_raw_input() -> parse_input() -> is_valid_move() -> place()

// AI player input flow (it is at present not possible for AI to choose a wrong coordinate):
find_best_move() -> InputType::Coord() -> place()
```

```rust
pub fn is_valid_move(&self, index: usize) -> Result<(), BoardError> {
    if index > self.len() - 1 {
        return Err(BoardError::OutOfBounds(index));
    } else if self.data[index] != CellState::Empty {
        return Err(BoardError::CellOccupied(index));
    }
    Ok(())
}
```

## What I learned

### **Implementing Minimax**
This was my first time implementing the minimax algorithm. The AI explores possible game states to pick the best move:

```rust
fn minimax(&self, board: &Board, players_info: &PlayersInfo, 
           depth: i8, is_maximizer: bool) -> i8
```
- Helped me understand recursive thinking and basic game theory

### **Rust Fundamentals**
- **Ownership & Borrowing**: Getting comfortable with Rust's memory management. I am still getting the hang of when to borrow and when to own.
- **Pattern Matching**: Using `match` for control flow and error handling -- I can't wait for `if let` [chains to be stabilized](https://rust-lang.github.io/rfcs/2497-if-let-chains.html).
- **Error Handling**: Working with `Result<T, E>`, which is a really useful concept that I far prefer over exception handling.
- **Traits**: Using trait objects like `Box<dyn PlayerController>` for polymorphism. I now know what dynamic dispatch is.
- **Custom Iterators**: Implementing iteration for `PlayerList`, just to get a hang of it.

### **Testing**
- **Testing**: Writing unit tests for different modules. I especially like how easy it is to do this natively in Rust, instead of installing jest/pytest etc etc.
- **TDD**: I know TDD is not hype these days, but I like understanding my domain first before I write code, and writing tests helps me think about what can go wrong.
- **Test Structure**: I experimented with making nested submodules to improve readability.
- **Unit Coverage**: Testing core functionality, though minimax was not tested. Not sure how to do that one effectively, and honestly I was too preoccupied with understanding the algorithm and how to make it work in Rust.

## How to run this

### Prerequisites
- Rust 1.70+ (uses Rust 2024 edition)

### Installation & Running

```bash
# Clone the repository
git clone https://github.com/oyvind-ege/tic-tac-toe-rust.git
cd tictactoe

# Run the game
cargo run

# Run tests
cargo test

# Run with optimizations
cargo run --release
```

### How to play

1. The game displays a numbered grid (0-8)
2. Enter a number to place your piece (X)
3. The AI will automatically make its move (Y)
4. Type `help` to see the board layout
5. Type `exit` to quit the game
