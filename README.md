## 📄➡️🖼️ Pdf2Image CLI
[![Rust](https://img.shields.io/badge/Language-Rust-000000?style=flat&logo=rust)](https://www.rust-lang.org/)
[![Crates.io](https://img.shields.io/badge/Published%20on-Crates.io-orange?style=flat&logo=rust)]([https://crates.io/](https://crates.io/crates/pdf2image-cli))
[![Docs.rs](https://img.shields.io/badge/Docs-Available-blue?style=flat&logo=readthedocs)]([https://docs.rs/](https://docs.rs/crate/pdf2image-cli/latest))
[![License](https://img.shields.io/crates/l/pdf2image-cli?style=flat)](https://crates.io/crates/pdf2image-cli)
[![Version](https://img.shields.io/crates/v/pdf2image-cli?style=flat)](https://crates.io/crates/pdf2image-cli)

A fast and lightweight **command-line tool** to convert PDF pages into high-quality images. Perfect for automation, batch processing, and backend pipelines. ⚡

---

🚀 Features

- 🖼️ Convert every PDF page into PNG/JPG images

- 🔍 Adjustable DPI for crisp output

- 📁 Specify custom output directory

- 🧰 Easy installation via Cargo

- ⚡ Built with Rust for maximum performance

## 📦 Installation

Install using Cargo:

```
cargo install pdf2image-cli
```

---

## 🛠️ Usage

You can specify the password using the `--password` argument.

### ▶️ Run from source


After cloning this repository:

```bash
cargo run -- --input Fuzzy.pdf --output-dir images --dpi 150 --format png
```

### ▶️ Run after installation

```bash
pdf2images --input Fuzzy.pdf --output-dir images --dpi 150 --format png
```
---