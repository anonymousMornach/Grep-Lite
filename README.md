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

## Prerequisites

Before you can build and use Grep-Lite, make sure you have the following tools installed on your system:

- [Git](https://git-scm.com/)
- [Rust](https://www.rust-lang.org/)

## Installation

Clone the repository and build the executable using the Rust compiler:

```bash
git clone https://github.com/anonymousMornach/Grep-Lite
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

To generate and view the documentation for Grep-Lite, use the following commands:

```bash
cargo doc --no-deps --open
```

### File Structure

- `main.rs`: Entry point for the application, handling command-line arguments and invoking the `grep_lite` function.
- `lib.rs`: Contains the implementation of the `grep_lite` function.
- `Cargo.toml`: Project configuration file with dependencies.

### Main Module

The `main.rs` file initializes the command-line interface using the `clap` library and invokes the `grep_lite` function with the provided arguments.

### Grep-Lite Function

The `grep_lite` function, implemented in the `lib.rs` file, performs a two-pass search through the specified file. It uses the `regex` library to find matches and then extracts contextual lines around each match. The results are then printed to the console, indicating the line number and content.

### Adding to Path
To add Grep-Lite to your system's PATH so that it can be used as a command globally, follow these steps for Windows, macOS, and Linux.

### Windows:

1. **Find the executable:**
   After building Grep-Lite, locate the executable file. In your case, it should be located at `target/debug/greplite.exe`.

2. **Add to PATH:**
   - Right-click on the Start menu and select "System".
   - Click on "Advanced system settings" on the left.
   - Click on the "Environment Variables..." button.
   - Under "System variables", find the "Path" variable, and click "Edit...".
   - Click "New" and add the directory path where `greplite.exe` is located.
   - Click "OK" to close each dialog box.

3. **Verify installation:**
   Open a new command prompt and type `greplite --version` to verify that Grep-Lite is now in your PATH.

### macOS/Linux:

1. **Find the executable:**
   After building Grep-Lite, locate the executable file. In your case, it should be located at `grep-lite/target/debug/greplite`.

2. **Add to PATH:**
   You can add the following line to your shell profile file (e.g., `~/.bashrc`, `~/.zshrc`, or `~/.bash_profile`):

   ```bash
   export PATH="<path>/grep-lite/target/debug:$PATH"
   ```

   Make sure to adjust the path accordingly.

   Then, run `source ~/.bashrc` (or the appropriate file based on your shell) to apply the changes.

3. **Verify installation:**
   Open a new terminal and type `greplite --version` to verify that Grep-Lite is now in your PATH.

Now, you should be able to use `greplite` as a command globally on your system. Remember to replace the path in the instructions with the correct path to your Grep-Lite executable.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details. Feel free to use, modify, and distribute this code for your own purposes.