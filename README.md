# rekv

Rekv is a lightweight, multi-threaded in-memory key-value store written in Rust. It supports simple operations like setting, getting, deleting, and incrementing values. It can be run in server mode or through an interactive CLI for quick testing.

## Installation

Install via Cargo:

```bash
cargo install rekv
```

## Usage

Start the server (default address is 127.0.0.1:4242):

```bash
rekv --addr 127.0.0.1:4242
```

Start the interactive CLI:

```bash
rekv --cli
```

## Commands

| Command              | Description                                             |
| -------------------- | ------------------------------------------------------- |
| `GET <key>`          | Retrieves the value of the given key.                   |
| `SET <key> <value>`  | Sets the value for the key. Value can be string/number. |
| `DEL <key>`          | Deletes the key and its value.                          |
| `ADD <key> <number>` | Adds a number to the existing numeric value.            |
| `.quit`              | Exits the interactive CLI.                              |

## License

MIT License.

