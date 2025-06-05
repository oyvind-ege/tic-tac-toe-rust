# Tic-Tac-Toe in Rust

A command-line tic-tac-toe game written in Rust, featuring human vs AI gameplay with a minimax algorithm implementation.

## Features

- **Human vs AI gameplay** with computer opponent, making optimal choices
- **Minimax algorithm** for optimal AI decision making
- **Modular architecture** with clear separation of concerns
- **Extensible design** supporting multiple player types and AI strategies

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

### Design Patterns & Principles

#### **Trait-Based Architecture**
- `PlayerController` trait enables polymorphic behavior for different player types
- `InputController` trait abstracts input handling for extensibility
- Clean separation between human and AI input processing

In the future, I would like to support multiplayer.

#### **Error Handling**
- Custom error types (`BoardError`, `InputError`) with descriptive messages
- Validation at multiple layers:
  - Input parsing validation
  - Game rule validation
  - Board state validation
- Occasional graceful error recovery with user-friendly feedback ;)

#### **Modular Design**
- I have tried to let each module have a single responsibility
- Largely clear interfaces between components
- Easy to extend with new player types or AI strategies

## Technical Highlights

### **Minimax AI Implementation**
The AI uses the minimax algorithm with game tree exploration to make optimal moves:

```rust
fn minimax(&self, board: &Board, players_info: &PlayersInfo, 
           depth: i8, is_maximizer: bool) -> i8
```

- Evaluates all possible game states
- Chooses moves that maximize AI advantage while minimizing opponent advantage
- Includes depth-based scoring for preferring quicker wins
- Demonstrates understanding of recursive algorithms and game theory

*The minimax implementation does not do alpha-beta pruning, yet!*

### **Input Validation**
I tried to keep this simple:

1. **Parsing Layer**: Converts string input to typed commands
2. **Game Logic Layer**: Validates moves against game rules
3. **Board Layer**: Ensures board state consistency

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

### **Somewhat Flexible Board Implementation**
- (Limited) support for arbitrary square board sizes (though optimized for 3x3)
- Comprehensive victory condition checking

### **Game State Management**
- Centralized game loop with turn-based processing
- Victory and draw detection
- Clean separation between game state and player actions
- Input retry loop ensuring valid moves before proceeding

*In the future: an event-based architecture.*

## Some of the skills this project requires

### **Rust**
- **Ownership & Borrowing**: Proper lifetime management without memory leaks
- **Pattern Matching**: Extensive use of `match` for control flow and error handling
- **Error Handling**: Idiomatic `Result<T, E>` usage throughout
- **Traits & Generics**: Polymorphic design with trait objects (`Box<dyn PlayerController>`)
- **Testing**: Unit tests with multiple test modules

### **Software Engineering**
- **Clean Architecture**: Clear separation of concerns across modules
- **SOLID Principles**: Single responsibility, open/closed, dependency inversion
- **Error Handling Strategy**: Comprehensive validation and user feedback
- **Extensible Design**: Easy to add new player types or AI strategies
- **Iterator Pattern**: Custom iterator implementation for `PlayerList`

### **Algorithms & Data Structures**
- **Minimax Algorithm**: Game theory implementation with depth consideration
- **Tree Traversal**: Recursive game state exploration
- **Mathematical Calculations**: Square root validation for board dimensions

### **Testing & Quality Assurance**
- **Unit Testing**: Comprehensive test coverage for core functionality
- **Edge Case Handling**: Tests for boundary conditions and error states
- **Test Organization**: Well-structured test modules (`get_row`, `get_column`, etc.)

## Code Quality Features

### **Input Validation Pipeline**
```rust
// Human player input flow:
get_raw_input() -> parse_input() -> is_valid_move() -> place()

// AI player input flow:
find_best_move() -> InputType::Coord() -> is_valid_move() -> place()
```

### **Error Propagation**
- `InputError::InvalidBoardError(BoardError)` wrapping preserves error context
- Display traits provide user-friendly error messages
- Input retry loops handle errors gracefully

### **Extensibility Points**
- `AIStrategy` enum allows multiple AI implementations
- `PlayerType` enum supports Local, AI, and Remote players
- Trait-based design enables easy addition of new player types

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

## Future Enhancements

The modular architecture makes it easy to extend:

- **Restarting the Game**: Allow players to restart the game, and have this part of the game loop
- **Network Multiplayer**: Add `PlayerType::Remote` implementation
- **AI Optimization**: Implement alpha-beta pruning 
- **GUI Interface**: Using SDL or Bevy
- **Save/Load**: Game state serialization
