# saturating_add_assign

This crate contains the `saturating_add_assign` macro, which looks like this:

```rust
macro_rules! saturating_add_assign {
    ($i:expr, $v:expr) => {{
        $i = $i.saturating_add($v)
    }};
}
```

There is nothing else in here and there never will be. This crate exists because
the [Agave](https://github.com/anza-xyz/agave) repo uses this macro everywhere and
I wanted to rip it out without heavy dependencies.
You should probably only use this crate if you were already using the
original `saturating_add_assign` macro.

You should pin the version to 1.0.0 in your Cargo.toml
file, because there isn't a clear reason why the four lines of code should
ever change, and if there is a new release it could mean I got hacked.

```toml
[dependencies]
saturating_add_assign = "=1.0.0"
```
