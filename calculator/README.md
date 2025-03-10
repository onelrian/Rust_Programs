## Simple CLI Calculator in Rust

This is a simple command-line calculator built with Rust. The program performs basic arithmetic operations and logs the history of calculations. It also has additional features such as exponentiation, factorial, and modulus operations. 
___

## Features
- Supports basic arithmetic operations: `+`, `-`, `*`, `/`
- Includes advanced features: exponentiation (`^`), factorial (`fact`), and modulus (`%`)
- History feature that logs past calculations
___

## Installation

### Clone the repository
```bash
git clone https://github.com/yourusername/rust-calculator.git
cd rust-calculator
```
___
### Building and Running with Docker

You can run the calculator inside a Docker container using the following steps:

1. **Build the Docker image**:
    ```bash
    docker build -t rust-calculator .
    ```

2. **Run the calculator inside the container**:
    Use `docker run` to execute commands. Here's an example to calculate the factorial of 5:
    ```bash
    docker run --rm rust-calculator fact 5
    ```


