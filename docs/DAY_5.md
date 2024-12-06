# Day 5

I went down the road of using a graph, but it didn't pan out with all the cycles in the graph.  But here's the examples nonetheless.

split a file into two sections

```rust
let input = std::fs::read_to_string(file).expect("File doesn't exist");
let mut sections = input.split("\n\n");
let section1 = sections.next().expect("Missing first section");
let section2 = sections.next().expect("Missing first section");

// ["47|53", "97|13", "97|61", ... ]
let first_section_vec: Vec<&str> = first_section.lines().collect();
// ["75,47,61,53,29", "97,61,53,29,13", ...]
let second_section_vec: Vec<&str> = second_section.lines().collect();
```

build a graph

```rust
use petgraph::algo::{has_path_connecting, is_cyclic_directed, toposort};
use petgraph::dot::{Config, Dot};

let mut g = petgraph::graph::DiGraph::<i32, ()>::new();
let n47 = g.add_node(47);
let n53 = g.add_node(53);
let n97 = g.add_node(97);
let n13 = g.add_node(13);
g.add_edge(n47, n53, ());
g.add_edge(n97, n13, ());
g.add_edge(n13, n47, ());
g.add_edge(n47, n97, ());

println!("{:?}", Dot::with_config(&g, &[Config::EdgeNoLabel]));

let is_cyclic = is_cyclic_directed(&g);
println!("is cyclic? {}", is_cyclic);

let result = toposort(&g, None);
match result {
    Ok(sorted_nodes) => println!("Topological order: {:?}", sorted_nodes),
    Err(_) => println!("Graph has cycles; no valid ordering exists."),
}

println!("Can 47 reach 53? {}", has_path_connecting(&g, n47, n53, None));
```
