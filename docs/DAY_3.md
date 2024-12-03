# Day 3

Regex (`cargo add regex`)

```rust
use regex::Regex;

fn process_line(line: &str) -> i32 {
    // notice capture groups around integers
    let re = Regex::new(r"mul\((\d+),\s*(\d+)\)").unwrap();

    // Iterate through all matches, parse and multiply numbers, and collect the results
    re.captures_iter(line)
        .map(|caps| {
            let x: i32 = caps[1].parse().expect("Not a number");
            let y: i32 = caps[2].parse().expect("Not a number");
            x * y
        })
        .sum() // Sum all the results
}
```
