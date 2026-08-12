# 💱 Currency Converter

A lightweight, fast command-line interface (CLI) written in **Rust** for fetching live exchange rates and converting currencies using the [Frankfurter API](https://api.frankfurter.dev/).

---

## ✨ Features

- 🚀 **Fast & Lightweight:** Built with Rust for safety, minimal memory usage, and high performance.
- 🔄 **Real-Time Data:** Queries the Frankfurter open-source exchange rate API (powered by European Central Bank data).
- 🛠️ **Simple CLI:** Swiftly fetch currency conversion rates directly from your terminal.
- 📦 **Type-Safe Parsing:** Leverages `serde` for robust and type-safe JSON deserialization.

---

## 📋 Prerequisites

Make sure you have the Rust toolchain (`cargo` and `rustc`) installed:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

```

---

## 📦 Installation & Setup

1. **Clone the repository:**
```bash
git clone https://github.com/whmee443/currency_convertor.git
cd currency_convertor

```


2. **Build the release binary:**
```bash
cargo build --release

```



The compiled executable will be located in `./target/release/currency_convertor`.

---

## 🚀 Usage

You can run the application directly with `cargo`:

```bash
cargo run -- [AMOUNT] <FROM_CURRENCY> <TO_CURRENCY>

```

Or run the compiled binary:

```bash
./target/release/currency_convertor [AMOUNT] <FROM_CURRENCY> <TO_CURRENCY>

```

### Examples

**Fetch exchange rate from USD to EUR:**

```bash
cargo run -- 1 USD EUR

```

**Convert 100 EUR to JPY:**

```bash
cargo run -- 100 EUR JPY

```


---

## 🌐 API Reference

This project utilizes the open-source **Frankfurter API**:

* **Base Endpoint:** `https://api.frankfurter.dev/v2`
* **Data Source:** European Central Bank (ECB)
* **Authentication:** None required (free and open access)

---

## 🛠️ Development

To check code formatting and verify build constraints during development:

```bash
# Check for compilation errors without building full binaries
cargo check

# Format code according to standard Rust style guidelines
cargo fmt

# Run Clippy lints
cargo clippy

```

---

## 📜 License

This project is licensed under the BSD 2-Clause License — see the [LICENSE](https://raw.githubusercontent.com/whmee443/currency_convertor/refs/heads/master/LICENSE) file for details.
