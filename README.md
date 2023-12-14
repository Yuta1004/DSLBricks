# DSL Generator with Syntax-Brick

## Requires

- make
- cargo
- Docker

## Command

```
// Setup(once)
$ make setup

// Test
$ make test

// Tools
$ cargo run --bin editor
$ cargo run --bin gen_template

// Example
$ cargo run --bin calc_interpreter
$ cargo run --bin minic_interpreter
```

## Crates

### bin

- [editor](./src/editor/README.md)
- [gen_template](./src/gen_template/README.md)
- calc_interpreter (example)
- minic_interpreter (example)

### lib

- [catalog](./catalog/README.md)
- [compiler](./compiler/README.md)
