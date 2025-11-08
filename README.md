## Pdf2Image
[![Rust](https://img.shields.io/badge/Language-Rust-000000?style=flat&logo=rust)](https://www.rust-lang.org/)
[![Crates.io](https://img.shields.io/badge/Published%20on-Crates.io-orange?style=flat&logo=rust)](https://crates.io/)
[![Docs.rs](https://img.shields.io/badge/Docs-Available-blue?style=flat&logo=readthedocs)](https://docs.rs/)
[![License](https://img.shields.io/crates/l/pdf2image-cli?style=flat)](https://crates.io/crates/pdf2image-cli)
[![Version](https://img.shields.io/crates/v/pdf2image-cli?style=flat)](https://crates.io/crates/pdf2image-cli)

A command-line application that converts PDF files into images.

## How to install

```
cargo install pdf2image-cli
```

## How to use

After cloning this repository

```
cargo run -- --input Fuzzy.pdf --output-dir images --dpi 150 --format png
```

or after installation 

```
pdf2images --input Fuzzy.pdf --output-dir images --dpi 150 --format png
```
