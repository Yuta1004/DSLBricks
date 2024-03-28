# DSLBricks

DSL Generator with Syntax-Brick

## Requires

- make
- cargo
- npm (optional / GUI Editor)
- hugo (optional / Docs)

## Command

### Setup (once)

```
$ make setup
```

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

```
// GUI Editor
$ cargo run --bin editor
```

### Docs

```
$ make docs
$ open docs/public/index.html
```

## Crates

### bin

- [editor](./src/editor/README.md)

### lib

- [compiler](./compiler/README.md)
- [library](./library/README.md)
