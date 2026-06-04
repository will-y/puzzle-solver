# Project for solving different puzzle games

## Layout
Each game will be first built in a playable way in a Rust crate. Then there will be a separate crate that can be used to solve a puzzle given an input. There will also be a web crate that allows interaction to other crates in the browser

### Star Puzzle
#### Model
##### Board
Things to consider: if going to do a tree solver, will need to copy the data a lot. Should probably only copy the State and not the Board Data.

###### Data
- public immutable Board Data (the colors on the board)
- private State (any placed stars maybe dots too, not sure yet)
###### Methods
- isValid()
- isSolved()
- placeStar(x, y)

