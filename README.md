# panic2abort

A small utility crate for translating panics to aborts in Rust programs.

## Example

```rust
extern crate panic2abort;

fn main() {
    // Required to get the linkage to work out, a bit of a hack...
    panic2abort::linkme();

    // This will abort the program instead of unwinding
    panic!("test");
}
```

## Drawbacks

* Only works on Linux (and perhaps other unix-ish systems)
* Requires some linkage hackery (see the `linkme` function)
* Really wants official support in the standard library and compiler for also
  not emitting landing pads!
