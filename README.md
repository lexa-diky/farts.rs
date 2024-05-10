# farts.rs

![Crates.io Total Downloads](https://img.shields.io/crates/v/farts?label=version)
![Crates.io Total Downloads](https://img.shields.io/crates/d/farts?logo=rust&label=crates.io%20downloads)
![GitHub top language](https://img.shields.io/github/languages/top/lexa-diky/farts?logo=rust)

![Crates.io License](https://img.shields.io/crates/l/farts?logo=apache)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/lexa-diky/farts/build?branch=main&logo=github)

Port of [fart.js](https://github.com/74656c/fart.js) library in Rust

## Installation

```bash
cargo add farts
```

## Usage

```rust
use farts;

fn main() {
    // you can discover more assets in farts::fart module
    farts::play(farts::fart::TOOT).unwrap();
}
```
