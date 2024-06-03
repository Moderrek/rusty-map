# HashMap Implementation in [Rust](https://rust-lang.org/)

RustyMap implementation.

## Performance

*Only on my machine*

| Map   | Elements  | Insert (ms)          | Get (ms)            |
| ----- | --------- | -------------------- | ------------------- |
| std   | 1 000     | 32ms                 | 15ms                |
| rusty | 1 000     | 18ms (1.7x faster)   | 5ms (3x faster)     |
| std   | 1 000 000 | 440ms                | 249ms               |
| rusty | 1 000 000 | 155ms (2.8x faster)  | 48ms (5.1x faster)  |
| std   | 5 000 000 | 2173ms               | 1550ms              |
| rusty | 5 000 000 | 1087ms (1.9x faster) | 243ms (6.3x faster) |


## Quick Start

```shell
cargo run
```

## References

- https://en.wikipedia.org/wiki/Hash_table
- http://www.cse.yorku.ca/~oz/hash.html
- https://en.wikipedia.org/wiki/Hash_collision
