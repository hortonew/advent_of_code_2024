# Day 4

Count overlapping strings in a larger string

Note: naive implementation with regex / str::matches doesn't count overlaps

```rust
fn count_overlapping_substrings(haystack: &str, needle: &str) -> usize {
    let mut count = 0;
    let mut start = 0;
    while let Some(pos) = haystack[start..].find(needle) {
        count += 1;
        start += pos + 1; // Move one character forward to allow overlap
    }
    count
}
```

Word Search

Search horizontal, vertical, then diagonal.  First do correctly spelled word, then its reverse.

```rust
let word = "XMAS";
let mut total_matches = search_grid(&grid, word);
let word: String = word.chars().rev().collect();
total_matches += search_grid(&grid, &word);
// ...

fn search_grid(grid: &[&str], word: &str) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut total = 0;

    // Search horizontally
    total += grid
        .iter()
        .map(|row| count_overlapping_substrings(row, word))
        .sum::<usize>();

    // Search vertically
    total += (0..cols)
        .map(|col| {
            let column: String = (0..rows).map(|row| grid[row].chars().nth(col).unwrap()).collect();
            count_overlapping_substrings(&column, word)
        })
        .sum::<usize>();

    // Search diagonally
    total += extract_diagonals(grid)
        .iter()
        .map(|diag| count_overlapping_substrings(diag, word))
        .sum::<usize>();

    total
}
```

Build subgrids from a grid

```rust
let input = std::fs::read_to_string(file).unwrap();
let grid: Vec<&str> = input.lines().collect();

let rows = grid.len();
let cols = grid[0].len();
let mut total_matches = 0;

for r in 0..rows - 2 {
    for c in 0..cols - 2 {
        // Check diagonals directly from the grid
        if is_x_mas(&grid[r..r + 3].iter().map(|line| &line[c..c + 3]).collect::<Vec<_>>()) {
            total_matches += 1;
        }
    }
}
```

Check a 3x3 Subgrid for an X of two possible words (MAS forward/reverse)

```rust
fn is_x_mas(subgrid: &[&str]) -> bool {
    // Extract diagonal elements
    let top_left_to_bottom_right = (
        subgrid[0].chars().nth(0).unwrap(),
        subgrid[1].chars().nth(1).unwrap(),
        subgrid[2].chars().nth(2).unwrap(),
    );
    let top_right_to_bottom_left = (
        subgrid[0].chars().nth(2).unwrap(),
        subgrid[1].chars().nth(1).unwrap(),
        subgrid[2].chars().nth(0).unwrap(),
    );

    // Define the valid "X" patterns
    let mas = ('M', 'A', 'S');
    let sam = ('S', 'A', 'M');

    // Both diagonals must match one of the valid patterns
    [mas, sam].contains(&top_left_to_bottom_right) && [mas, sam].contains(&top_right_to_bottom_left)
}
```
