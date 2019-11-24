# balanced-group-shuffle
Shuffle given groups by dispersing the values with slight randomness

This library is inspired by shuffling songs by artists in a playlist. True random shuffle would 
not take into account the incostistent representation of songs by one artist compared to others.

The `BalancedGroupShuffle` disperses the values of one group as far away from each other with small
5% randomness in the spread.

Let's consider this example with groups a: {a1, a2}, b: {b1, b2, b3}, c: {c1, c2, c3, c4}, d: {d1}

The resulting disperse could look like the following with total of 10 items
```
a: a2  -  -  -  -  -  a1  -  -  -

b: b1  -  -  b3 -  -  -  b2  -  - 

c: -  c4  -  -  c1 -  c2  -  -  c3 

d: -  -  d1  -  -  -  -   -  -  - 
```

And when collected the items are taken position by position to the resulting vector:

```
=> [a2, b1, c4, d1, b3, c1, a1, c2, b2, c3]
```


## Usage

```rust
let mut groups = BalancedGroupShuffle::new();
groups.insert("a", "a1");
groups.insert("a", "a2");
groups.insert("b", "b1");
groups.insert("b", "b2");
groups.insert("b", "b3");
groups.insert("c", "c1");
println!("{:?}", groups.shuffle());
// ["a2", "b1", "b3", "c1", "a1", "b2"]
```