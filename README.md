# Rorka

**Rorka** is a game engine written in Rust, a rewrite of
[Borka](https://github.com/JonatanBengtsson94/borka), a game engine written in C.

## Local Development

### Prerequisites

- A Rust toolchain, installed with [rustup](https://rustup.rs)

### Quickstart

Build and run the demo:

```sh
cargo demo
```

`cargo demo` is an alias for `cargo run --example demo`, defined in
`.cargo/config.toml`.

### Logging

Rorka logs through the [`log`](https://docs.rs/log) crate. Games need to install
a logger, such as [`env_logger`](https://docs.rs/env_logger), to see its output.
