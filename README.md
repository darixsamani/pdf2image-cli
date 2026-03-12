## 📄➡️🖼️ Pdf2Image CLI
[![Rust](https://img.shields.io/badge/Language-Rust-000000?style=flat&logo=rust)](https://www.rust-lang.org/)
[![Crates.io](https://img.shields.io/badge/Published%20on-Crates.io-orange?style=flat&logo=rust)](https://crates.io/crates/pdf2image-cli)
[![Docs.rs](https://img.shields.io/badge/Docs-Available-blue?style=flat&logo=readthedocs)](https://docs.rs/crate/pdf2image-cli/latest)
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

---

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

### 📥 Installing `libpdfium.so`

Important to note that we don't need a shared binary file in the case that we have **clang** installed

To use pdfium-render, you need the `libpdfium.so` binary for your system.

1. Clone the official pdfium-binaries repository:

```bash
git clone https://github.com/bblanchon/pdfium-binaries
cd pdfium-binaries
```

2. Build the library according to your OS and architecture.

For example, on **Linux x64**: 

```bash
./build.sh linux x64
```

3. After the build completes, the compiled `libpdfium.so` will be available inside the build output directory.

Copy it into your project’s expected library path (e.g., `./pdfium/` or `/usr/lib/`).
