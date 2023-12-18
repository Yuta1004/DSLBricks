# DSL Generator with Syntax-Brick

## Requires

- make
- cargo
- npm

## Command

```
// Docs(rustdoc) => http://localhost:5555/catalog/index.html
$ make -C docs/rustdoc web

// Test
$ make test

// Tools
$ cargo run --bin editor
$ cargo run --bin gen_template

// Example
$ cargo run --bin calc_interpreter
$ cargo run --bin minic_interpreter
```

## Docs

- [catalog](./docs/catalog/README.md)
- [rustdoc](./docs/rustdoc/README.md)

## Crates

### bin

- [editor](./src/editor/README.md)
- [gen_template](./src/gen_template/README.md)
- [rustdoc_web](./docs/rustdoc/tools/rustdoc_web/README.md)
- calc_interpreter (example)
- minic_interpreter (example)

### lib

- [catalog](./catalog/README.md)
- [compiler](./compiler/README.md)
