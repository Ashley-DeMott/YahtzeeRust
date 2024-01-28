# Yahtzee - Rust
## Description
Created a simple version of Yahtzee using the terminal. The player can choose to roll die, freeze die, select a score section, or quit. When all the score sections have been filled, the game will be over and the total points will be displayed. The player can roll up to three times before they must choose a score section (but may do so earlier).

[Software Demo Video](https://www.youtube.com/watch?v=cZN42zq9GlU)

## Important aspects of Rust
- Use 'cargo build' and 'cargo run'
- Dependencies on other libraries are added to .toml and downloaded on 'cargo build'
- Variables must be declared mutable (aka 'mut') on declaration
- No classes, uses structs (collection of attributes) and traits (methods) that structs can have
- Default values for custom structs can be set by implementing 'Default' trait

## Development Environment
- Visual Studio Code
- Rust version 1.75.0
- Rust (v1.0.0) and rust-analyzer (v0.3.1815) extensions from VS Code

## Useful Websites
- [Rust Download](https://www.rust-lang.org/learn/get-started)
- [Creating Vectors of Structs](https://users.rust-lang.org/t/can-we-make-vector-of-structs-if-yes-than-how-to-make-and-use-it/19476)
- [Object Oriented Design with Rust](https://stevedonovan.github.io/rust-gentle-intro/object-orientation.html)

## Future Work
- Use GUI libraries for output
- Move scorecard creation to its own function
- Move independent functions to structs with traits (Scorecard struct that implements methods such as is_full/empty_section, get_score, display_scorecard)
- Ability to work for different numbers of Die with different values (ex: 10 d20 die)
