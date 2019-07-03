# Tic Tac Rust
A Tic tac toe game manager built in rust and packaged to npm as a WebAssembly [wasm-pack](https://github.com/rustwasm/wasm-pack) package

----
# What is Tic Tac Toe
see [Wikipedia](https://en.wikipedia.org/wiki/Tic-tac-toe)

> Tic-tac-toe (American English), noughts and crosses (British English), or Xs and Os is a paper-and-pencil game for two players, X and O, who take turns marking the spaces in a 3×3 grid. The player who succeeds in placing three of their marks in a horizontal, vertical, or diagonal row wins the game.

----
# Installation
1. `$ npm install tic-tac-rust`
2. In your js/ts: 
``` 
const { State } = await import('tic-tac-rust');
```

----
# Usage
### Creating a new game state

```
 const gameState = State.new(difficulty);
// difficulty is 0 (Easy), 1 (Normal) or 2 (Computer will be unbeatable)
```

----
### Updating the board
```
gameState.update_board(index, character)
// index is 0 - 8 starting top left, character is 'x', 'o' or '0'
```
----
### Checking for win/loss/tie
You can check for a win or a loss using the is_win function:
```
const xWon = gameState.is_win('x'); // xWon would be true if x won
```
```
const oWon = gameState.is_win('o'); // oWon would be true if o won
```

or check for a tie:
```
const isTie = gameState.is_tie() // isTie is true if it is a tie
```
----
### Ask for computer's next move
This is based on the difficulty the state was created using
```
const index = gameState.next_move(x_or_o) 
// x_or_o is 'x' if computer is 'x' and 'o' if computer is 'o'
```
----
## Built Using
* [Rust](https://www.rust-lang.org/) :heart: 
* [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen)
* [Rand](https://github.com/rust-random/rand)

----
## Contribution
This was a quick side project I did to while learning WebAssembly (This will Soon be on my personal site!).
If anyone ends up using it and wants to contribute, feel free to create a PR :grin:
