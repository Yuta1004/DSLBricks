# DSLBricks

DSL Generator with Syntax-Brick

## Requires

- cargo
- make (optional / GUI Editor, Docs)
- npm (optional / GUI Editor)
- hugo (optional / Docs)

## Command

### Test

```
$ cargo test
```

### Examples

```
$ cargo run --bin calc_interpreter
$ cargo run --bin minic_interpreter
```

### Tools

#### GUI Editor

```
$ make -C src/editor setup
$ cargo run --bin editor
```

### Docs

```
$ make -C docs
$ open docs/public/index.html
```

## Crates

### bin

- [editor](./src/editor/README.md)

### lib

- [compiler](./compiler/README.md)
- [library](./library/README.md)
