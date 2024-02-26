# DSLBricks

DSL Generator with Syntax-Brick

## Requires

- make
- cargo
- npm (for GUI Editor)

## Command

### Test

```
$ make test
```

### Exaples

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

## Crates

### bin

- [editor](./src/editor/README.md)
- [rustdoc_web](./src/rustdoc_web/README.md)
- calc_interpreter (example)
- minic_interpreter (example)
- test_default (test)
- test_with_serde (test)

### lib

- [compiler](./compiler/README.md)
- [library/catalog](./library/catalog/README.md)
