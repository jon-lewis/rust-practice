Project created by:

```
cargo new hello_cargo
```

Build in debug mode without running.

```
cargo build
./target/debug/hello_cargo
```

Release build.

```
cargo build --release
./target/release/hello_cargo
```

Build and run.

```
cargo run
cargo run --release
```

Check if it compiles without building. This is often faster than building.

```
cargo check
```