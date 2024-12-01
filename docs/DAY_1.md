# Day 1

Read file

```rust
let input = std::fs::read_to_string(file).expect("Failed to read file");
let lines: Vec<&str> = input.lines().collect();
```

Create tuples of i32s from lines in file

```rust
for line in lines {
    let numbers: Vec<i32> = line
        .split_whitespace()
        .map(|num| num.parse().expect("Failed to parse number"))
        .collect();
}
```

Sort vectors in place

```rust
let mut left_vec = Vec::new();
left_vec.sort()
```

Count values in a vector with

```rust
let mut left_vec = Vec::new();
let mut right_vec = Vec::new();
for &left_num in left_vec {
    let count = right_vec.iter().filter(|&num| *num == left_num).count();
}
```

Write a test

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_total_distance() {
        assert_eq!(1, 1);
    }
```
