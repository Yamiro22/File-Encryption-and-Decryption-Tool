
---

# File Encryption and Decryption Tool

This tool is a command-line interface (CLI) application written in Rust, capable of encrypting and decrypting files using AES-256-CBC symmetric encryption. It's designed to be simple, efficient, and easy to use for basic file encryption needs.

## Getting Started

### Prerequisites

- Rust programming language environment.
- OpenSSL library.

### Installation

1. **Clone the repository or download the source code**.
2. **Navigate to the project directory** where the `Cargo.toml` file is located.

### Building the Tool

Run the following command in the project root directory:

```bash
cargo build --release
```

The executable will be generated in `target/release/`.

## Usage

The tool can be run from the command line with the following syntax:

```bash
./target/release/encryption_tool <command> <key> <file>
```

- `<command>` - Either `encrypt` or `decrypt`.
- `<key>` - The encryption/decryption key. Any string can be used as the key.
- `<file>` - Path to the file that needs to be encrypted or decrypted.

### Examples

- Encrypting a file:

  ```bash
  ./target/debug/encryption_tool encrypt mysecretkey /home/yamiro22/RustroverProjects/encryption_tool/new.txt

  ```

- Decrypting a file:

  ```bash
  ./target/debug/encryption_tool decrypt mysecretkey /home/yamiro22/RustroverProjects/encryption_tool/new.txt

  ```

## Important Notes

- The tool uses AES-256-CBC encryption and requires a 32-byte key. It automatically hashes any given key to meet this requirement.
- The original file will be overwritten with its encrypted or decrypted version. Ensure you have backups of important files.
- This tool is intended for educational or personal use. For production-grade encryption needs, consider more comprehensive solutions.

## Contributing

Feel free to fork the repository and submit pull requests. For major changes, please open an issue first to discuss what you would like to change.

## License

This project is open-source and available under the [MIT License](LICENSE.md).

---

