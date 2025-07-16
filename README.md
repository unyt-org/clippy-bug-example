# Clippy Bug Minimal Reproducible Example

When running clippy, the generated output contains invalid syntax.
```rust
pub struct Flags {
    val: u8,
}
```
gets transformed to 

```rust
pub struct Flags {
    al: u,
}
```

1. Install cargo nightly `1.90.0-nightly (3014e79f9 2025-07-15)`
2. Run `cargo clippy`
   Output
   ```diff
   - val: u8,
   + al: u,
   ```