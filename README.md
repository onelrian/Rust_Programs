# Rust Basics Repository

Welcome to the **Rust Basics** repository! This repository contains a collection of basic Rust code examples and exercises to help you get started with the Rust programming language. Whether you're new to Rust or looking to refresh your skills, this repo is a great place to begin.

---
## Getting Started

### Prerequisites

Before you begin, ensure you have the following installed on your system:

1. **Rust**: If you don't have Rust installed, you can install it by following the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

   After installation, verify it by running:

   ```bash
   rustc --version
   ```

2. **Git**: Ensure you have Git installed to clone the repository. You can download it from the [official Git website](https://git-scm.com/).

   Verify Git installation:

   ```bash
   git --version
   ```

---

### Cloning the Repository

To get started, clone this repository to your local machine using the following command:

```bash
git clone https://github.com/onelrian/Rust_Programs.git
```

Replace `your-username` with your GitHub username or the appropriate repository URL.

After cloning, navigate to the repository directory:

```bash
cd Rust_Programs
```

---

## Running the Examples

Each Rust file in the `src/` directory is a standalone example. To run a specific example, use the following command:

```bash
cargo run --bin <filename_without_extension>
```

For example, to run the `hello_world.rs` example:

```bash
cargo run --bin hello_world
```

This will compile and execute the code, displaying the output in your terminal.


---

Happy coding in Rust! 🦀