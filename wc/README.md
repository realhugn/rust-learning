# Rust Learning Project: Word Count (wc)

This project is a Rust implementation of the classic Unix `wc` (word count) command. The `wc` command is used to count the number of lines, words, and bytes in a file. This project aims to provide a hands-on learning experience with Rust by building a useful command-line tool.

## Features

- Count the number of lines in a file
- Count the number of words in a file
- Count the number of bytes in a file
- Support for multiple input files
- Display results in a user-friendly format

## Getting Started

To get started with this project, you will need to have Rust installed on your system. You can install Rust by following the instructions on the [official Rust website](https://www.rust-lang.org/).

## Usage

To use the `wc` command, run the following command in your terminal:

```sh
cargo run -- <file1> <file2> ...
```

Replace `<file1>`, `<file2>`, etc., with the paths to the files you want to analyze.
## Available Options

The `wc` command supports the following options:

- `-l` or `--lines`: Count the number of lines in the file(s).
- `-w` or `--words`: Count the number of words in the file(s).
- `-c` or `--bytes`: Count the number of bytes in the file(s).
- `-h` or `--help`: Display help information about the command usage.

You can use these options in combination with the file paths. For example, to count the number of lines and words in a file, you can run:

```sh
cargo run -- -l -w <file1>
```
## Contributing

Contributions are welcome! If you have any suggestions or improvements, please open an issue or submit a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.