# Simple sudoku solver written in Rust
A simple app to solve sudoku puzzles using multithreading.
## Usage:
Have all your sudoku puzzles in one file. Each puzzle should be on it's own line. Each line should contain 81 characters, as if the rows of the sudoku were placed one after another starting from the topmost row. Empty spots should be represented as dots. Example:
`.5..83.17...1..4..3.4..56.8....3...9.9.8245....6....7...9....5...729..861.36.72.4`
The output will be redirected in the same format to the file you specified (see `$ sudoku --help`).
