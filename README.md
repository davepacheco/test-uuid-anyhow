Output of this program:

```rust
top-level (anyhow) error:
     formatted: parsing
    alt format: parsing: invalid length: expected one of [36, 32], found 3: invalid length: expected one of [36, 32], found 3
anyhow debug format:
parsing

Caused by:
    0: invalid length: expected one of [36, 32], found 3
    1: invalid length: expected one of [36, 32], found 3

CAUSE CHAIN
chain 0: Error { context: "parsing", source: Error(Parser(InvalidLength { expected: Any([36, 32]), found: 3 })) }
     formatted: parsing
    alt format: parsing


chain 1: Error(Parser(InvalidLength { expected: Any([36, 32]), found: 3 }))
     formatted: invalid length: expected one of [36, 32], found 3
    alt format: invalid length: expected one of [36, 32], found 3


chain 2: InvalidLength { expected: Any([36, 32]), found: 3 }
     formatted: invalid length: expected one of [36, 32], found 3
    alt format: invalid length: expected one of [36, 32], found 3

```

See uuid-rs/uuid#533 for details.
