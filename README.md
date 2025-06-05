# My Tic-Tac-Toe Game in Rust

A command-line tic-tac-toe game I built while learning Rust. You can play against an AI opponent that uses the minimax algorithm to make optimal moves.

## What It Does

- **Play against the computer** - the AI uses minimax to make good moves
- **Clean code structure** - I tried to organize things in a way that makes sense
- **Room to grow** - designed as best as I could, so I can add new features later

## Architecture

### Core Components

```
src/
├── main.rs              # Entry point and game initialization
├── gamestate/           # Game state management and main game loop
├── board.rs             # Board representation and game logic
├── player/              # Player management and types
│   ├── playerbase.rs    # Player definitions and controller traits
│   └── playerlist.rs    # Player collection and iteration
├── controller.rs        # Input handling and validation traits
└── ai/                  # AI implementation
    ├── mod.rs          # AI strategy enumeration
    └── minimax.rs      # Minimax algorithm implementation
```

### How I Structured Things

#### **Using Traits for Flexibility**
Traits is the Rust equivalent of interfaces. I have a couple in my code: 

- `PlayerController` trait lets me handle human and AI players the same way
- `InputController` trait makes it easy to add different input methods later
- Kept human and AI logic separate but consistent

This might be overengineered, and I am not entirely convinced myself, but it made for fun learning.


#### **Enums**
I love enums. Here are some of mine:

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

#### **Error Handling**
- Custom error types (`BoardError`, `InputError`) with helpful messages
- Input validation happens primarily on the human player side when parsing user input
    - I assume AI never suggests and out-of-bounds move, or a move on a non-empty cell
- Tries to give useful feedback when things go wrong

There is still some work to be done here. I am overall not happy with the error handling patterns.

#### **Example: Handling User Input**
I kept the validation flow straightforward:

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

#### **Keeping Things Organized**
- Each module focuses largely on one main thing
- Interfaces between parts are reasonably clean
- Should be straightforward to add new player types or AI strategies

I am still learning how to structure my Rust projects, but this works ish.

## What I Learned Building This

### **Implementing Minimax**
This was my first time implementing the minimax algorithm. The AI explores possible game states to pick the best move:

```rust
fn minimax(&self, board: &Board, players_info: &PlayersInfo, 
           depth: i8, is_maximizer: bool) -> i8
```

- Looks ahead at all possible moves and counter-moves
- Picks moves that are best for the AI while assuming the human plays optimally
- Prefers quicker wins over slower ones
- Helped me understand recursive thinking and basic game theory

*I haven't added alpha-beta pruning yet - that's on my list!*

### **Board Design**
- Should, without too much additional code, work with different square board sizes (though I only tested 3x3)

### **Game Flow**
- Main game loop handles turns pretty ok, though an even-based approach would be better (and necessary once we get to multiplayer and graphics)
- Detects wins and draws properly
- Keeps game state separate from player logic
- Lets players retry when they enter invalid moves

*I'm thinking about trying an event-based approach next time.*

## What This Project Helped Me Practice

### **Rust Fundamentals**
- **Ownership & Borrowing**: Getting comfortable with Rust's memory management
- **Pattern Matching**: Using `match` for control flow and error handling
- **Error Handling**: Working with `Result<T, E>`, which is a cool concept
- **Traits**: Using trait objects like `Box<dyn PlayerController>` for polymorphism
- **Testing**: Writing unit tests for different modules

### **Code Organization**
- **Modular Design**: Separating concerns across different modules, pretty much
- **Clean Interfaces**: Keeping dependencies clear and minimal - I'd rather do hard things myself than use other peoples work
- **Error Strategy**: Building validation and user feedback into the design - largely
- **Extensibility**: Making it easier to add features later
- **Custom Iterators**: Implementing iteration for `PlayerList` - that was fun.

### **Algorithms**
- **Minimax**: My first real game AI algorithm - took me a while to understand!
- **Recursion**: Exploring game trees and state spaces
- **Validation Logic**: Checking board dimensions and move validity

### **Testing Approach**
- **Unit Coverage**: Testing core functionality, though minimax was not tested
- **Edge Cases**: Handling boundary conditions and error states
- **Test Structure**: Organizing tests into logical modules

## How to Run This Thing

### Prerequisites
- Rust 1.70+ (uses Rust 2024 edition)

### Installation & Running

```bash
# Clone the repository
git clone <repository-url>
cd tictactoe

# Run the game
cargo run

# Run tests
cargo test

# Run with optimizations
cargo run --release
```

### How to Play

1. The game displays a numbered grid (0-8)
2. Enter a number to place your piece (X)
3. The AI will automatically make its move (Y)
4. Type `help` to see the board layout
5. Type `exit` to quit the game

## What I Want to Build Next

The way I structured things should make these additions pretty straightforward:

- **Game Restart**: Let players start a new game without exiting
- **Network Play**: Add remote multiplayer (this one might be tricky!)
- **Better AI**: Implement alpha-beta pruning to make minimax faster
- **GUI Version**: Maybe try Bevy or another Rust graphics library
- **Save Games**: Serialize game state so you can resume later
