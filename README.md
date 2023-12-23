# Grep-Lite

Grep-Lite is a simple command-line utility that searches for patterns in a given file. It provides a lightweight alternative to the traditional grep tool, allowing users to find and display lines containing a specified pattern, along with contextual lines around the match.

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
- [Examples](#examples)
- [Documentation](#documentation)
  - [File Structure](#file-structure)
  - [Main Module](#main-module)
  - [Grep-Lite Function](#grep-lite-function)
- [License](#license)

## Installation

Clone the repository and build the executable using the Rust compiler:

```bash
git clone https://github.com/your_username/grep-lite.git
cd grep-lite
cargo build --release
```

## Usage

Grep-Lite is designed to be used from the command line. It takes a pattern to search for and a file to search within. The context_lines parameter allows users to specify the number of lines displayed around each match.

```bash
grep-lite <pattern> <filename>
```

### Options

- `--version`: Display the version information.
- `--help`: Display a help message.

## Examples

Search for the pattern "error" in the file "logfile.txt" with one context line:

```bash
./grep-lite error logfile.txt
```

Search for the pattern "TODO" in all Rust files within the current directory:

```bash
./grep-lite TODO *.rs
```

## Documentation

### File Structure

- `main.rs`: Entry point for the application, handling command-line arguments and invoking the `grep_lite` function.
- `lib.rs`: Contains the implementation of the `grep_lite` function.
- `Cargo.toml`: Project configuration file with dependencies.

### Main Module

The `main.rs` file initializes the command-line interface using the `clap` library and invokes the `grep_lite` function with the provided arguments.

### Grep-Lite Function

The `grep_lite` function, implemented in the `lib.rs` file, performs a two-pass search through the specified file. It uses the `regex` library to find matches and then extracts contextual lines around each match. The results are then printed to the console, indicating the line number and content.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details. Feel free to use, modify, and distribute this code for your own purposes.