# STIX Hackathon

This repository includes experiments on leveraging Rust's type system to
represent the [STIX standard][stix-standard].

## Usage

1. Make sure to [have a Rust compiler](https://www.rust-lang.org/tools/install)
2. Watch the sources for file change, re-compile, and run immediately

   ```sh
   cargo install cargo-watch         # install the `watch` util
   cargo watch -x run
   ```

[stix-standard]: https://docs.oasis-open.org/cti/stix/v2.1/os/stix-v2.1-os.html
