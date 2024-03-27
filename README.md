# DSLBricks

DSL Generator with Syntax-Brick

## Requires

- make
- cargo
- npm (optional / GUI Editor)
- docker (optional / Docs)

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

```
// GUI Editor
$ cargo run --bin editor

// Rustdoc on Web => http://localhost:5555/catalog/index.html
$ cargo run --bin gen_template
```

### Docs

```
// Docs on Web => http://localhost:1313
$ make -C docs
```

## Crates

### bin

- [editor](./src/editor/README.md)
- [rustdoc_web](./src/rustdoc_web/README.md)

### lib

- [compiler](./compiler/README.md)
- [library](./library/README.md)
