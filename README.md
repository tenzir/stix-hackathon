# STIX Hackathon

This repository includes experiments on leveraging Rust's type system to
represent the [STIX standard][stix-standard].

## The Plan

The goal is to translate the standard into type definitions. For the scope of
this hackathon, we begin with [STIX Domain Objects (SDOs)][sdos].

There exists a minimal end-to-end example for the [Attack Pattern
SDO][sdo-attack-pattern]:

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

**Note**: We're using [Serde's internally
tagged](https://serde.rs/enum-representations.html#internally-tagged) decoders.
Which means we turn `{type: foo, ...}` into `Foo(...)`, so we have enums and
can pattern match.

## Usage

1. Make sure to [have a Rust compiler]("https://www.rust-lang.org/tools/install")
2. Watch the sources for file change, re-compile, and run immediately

   ```sh
   cargo install cargo-watch         # install the `watch` util
   cargo watch -x run
   ```

[stix-standard]: https://docs.oasis-open.org/cti/stix/v2.1/os/stix-v2.1-os.html
[sdos]: https://docs.oasis-open.org/cti/stix/v2.1/os/stix-v2.1-os.html#_nrhq5e9nylke
[sdo-attack-pattern]: https://docs.oasis-open.org/cti/stix/v2.1/os/stix-v2.1-os.html#_axjijf603msy
