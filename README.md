# Gematria-rs

[![Crates.io](https://img.shields.io/crates/v/gematria-rs.svg)](https://crates.io/crates/gematria-rs)
[![Documentation](https://img.shields.io/docsrs/gematria_rs/latest)](https://docs.rs/gematria_rs/latest/gematria_rs/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

`Gematria-rs` is a Rust library designed for calculating Gematria values in Hebrew texts. It supports multiple calculation methods and is suitable for analyzing texts like the Hebrew Bible. The library includes a command-line interface (CLI) for easy interaction.

## Features

- **Multiple Gematria Calculation Methods**: Supports various Gematria methods such as Mispar Hechrechi, Mispar Gadol, Mispar Katan, etc.
- **Hebrew Text Analysis**: Tailored for processing Hebrew scripts, including handling vowelizations (nikkud).
- **CLI for Easy Usage**: A user-friendly command-line interface for performing Gematria calculations on texts.
- **Text File Processing**: Ability to process entire text files and group words based on their Gematria values.
- **Flexible Data Handling**: Designed to handle words with different vowelizations as unique entries.
- **Customizable**: Easy to integrate into larger projects and customize for specific analytical needs.

## Usage

### As a Library
Include `Gematria-rs` in your Rust project by running:

```bash
cargo add gematria_rs
```

Or adding it to your `Cargo.toml`:

```toml
[dependencies]
gematria_rs = "0.1.1"
```

Use it in your project:

```rust
use gematria_rs::GematriaContext;

let gematria_context = GematriaContext::default();

let value = gematria_context.calculate_value("שלום");
println!("Gematria value: {}", value);
```

> You can use the `gematria_rs::GematriaBuilder` to change the default settings for the context easily.

### As a CLI Tool
To use the CLI tool, clone the repository and build the project:

```bash
git clone https://github.com/MadBull1995/gematria-rs.git
cd gematria-rs
cargo build --release
```

Run the CLI:

```bash
./target/release/gematria [COMMAND] [OPTIONS]
```

## CLI

The CLI provides the following functionalities:

- **Calculate Gematria Value**: Calculate the Gematria value of a given Hebrew word or phrase.
- **Group Words by Gematria**: Analyze a text file and group words based on their Gematria values.

Use `--help` to see all available commands and options.

### Examples
> You can use the full text file of the Hebrew Bible at [`/data/hebrew-all.txt`](data/hebrew-all.txt)

Group all words that equals the same value in "Standard" gematria from `stdin`:
```bash
./target/release/gematria group-words < ./data/hebrew-all.txt
```
Or from argument:
```bash
./target/release/gematria group-words "נכנס יין יצא סוד"
#  70 -> יין, סוד
```

## Development

### Setting Up the Development Environment
Ensure you have Rust and Cargo installed. Clone the repository and you can start contributing to `Gematria-rs`.

### Running Tests
To run tests, use:

```bash
cargo test
```

### Building Documentation
To build the documentation locally, including the `Katex` math typesetting for Gematria method explanations, run:

```bash
RUSTDOCFLAGS="--html-in-header src/docs-header.html" cargo doc --no-deps --open
```

## Contributing
Contributions to `Gematria-rs` are welcome! Whether it's improving documentation, adding more Gematria methods, or enhancing the CLI tool, your input is valuable.

<!-- Please read our contributing guidelines (LINK CONTRIBUTING GUIDELINES) to get started. -->

## License
`Gematria-rs` is licensed under [MIT License](LICENSE).