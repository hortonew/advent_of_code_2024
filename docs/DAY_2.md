# Day 2

Two pointers

```rust
let levels: Vec<i32> = line
    .split_whitespace()
    .map(|num| num.parse().expect("Failed to parse number"))
    .collect();

let mut left = 0;
let mut right = 1;
while right < levels.len() {
    right += 1;
    left = right - 1;
}
```

Remove an element from cloned vector

```rust
for i in 0..levels.len() {
    let mut modified_levels = levels.clone();
    modified_levels.remove(i);
}
```