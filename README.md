# STIX

## The Plan
This repo holds some scaffolding to a typed definition of the [STIX Standard](https://docs.oasis-open.org/cti/stix/v2.1/os/stix-v2.1-os.html#_axjijf603msy).

The plan is to take the "Stix Domain Objects" (chapter 4 in the standard), and
make a typed definition to it. 
Currently the `AttackPattern` has been fleshed out. Please find the examples
here:
- JSON / JSON parsing call -> `main.rs`
- Main `STIXDomainObject` implementation -> `sdo/mod.rs`
- Fleshed out `AttackPattern` -> `sdo/attack_pattern.rs`

The `Campaign`, `CourseOfAction`, and `Grouping` are ready to be picked up. To
add a new element to the enum:
1. Create the module (`sdo/{your_module}.rs`)
2. Declare the module in `sdo/mod.rs` (adding: `pub mod {your_module}`)
3. Add the enum, making sure to rename the tag like the others;

```rust
#[serde(rename = "your-module")]
YourModule(your_module::YourModule),
```

4. Test it with some JSON.

**NOTE** - We're using [Serde's internally tagged](https://serde.rs/enum-representations.html#internally-tagged) decoders. Which means we turn `{type: foo, ...}` into `Foo(...)`,
    so we have enums and can pattern match.

# Running it
- Make sure to [have a Rust compiler]("https://www.rust-lang.org/tools/install")
- Watch the sources for file change, re-compile and run immediately

```
cargo install cargo-watch         # install the `watch` util
cargo watch -x run
```
