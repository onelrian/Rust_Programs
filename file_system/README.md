# File System Simulation in Rust

This project is a simple file system simulation implemented in Rust. It allows users to create, delete, and list files and folders while leveraging Rust traits, iterators, `dyn` dispatch, and other key concepts.

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Installation](#installation)

## Overview

The file system simulation project provides a basic implementation of a file system with the following features:
- Create and manage files and folders.
- Add items to folders.
- Calculate the total size of folders.
- Display the contents of folders and files.

## Features

- **File and Folder Creation**: Create files and folders with names, sizes, and content.
- **Item Management**: Add files and folders to other folders.
- **Size Calculation**: Calculate the total size of a folder and its contents.
- **Display**: Display the contents of files and folders.
- **Iteration**: Iterate over the items in a folder using an iterator.

## Installation

To run this project, you need to have Rust installed on your system. If you don't have Rust installed, you can download it from the official Rust website: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

1. Clone the repository:
   ```sh
   git clone https://github.com/onelrian/Rust_Programs/tree/main/file_system.git
```

Navigate to the project directory:
```
cd file_system
```
Build and run the project:

```rust
cargo run
```
