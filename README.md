# jwt-cli

Small Rust command-line tool for decoding a JWT payload from `stdin` to `stdout`.

## Features

- Reads a JWT from standard input
- Decodes the payload segment using base64url
- Prints the decoded payload as UTF-8 text
- Exits with a non-zero status and an error message for invalid input

## Requirements

- Rust toolchain with `cargo`

## Installation

Build the binary locally:

```bash
cargo build --release
```

Run it from the built artifact:

```bash
./target/release/jwt
```

Or install it into your Cargo bin directory:

```bash
cargo install --path .
```

## Usage

Pipe a JWT into the CLI:

```bash
echo 'eyJhbGciOiJub25lIiwidHlwIjoiSldUIn0.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIn0.' | jwt
```

Example output:

```json
{"sub":"1234567890","name":"John Doe"}
```

You can also run it without installing:

```bash
echo 'TOKEN' | cargo run --quiet
```

## Error Handling

The tool returns an error when:

- `stdin` is empty
- The token does not contain exactly three dot-separated segments
- The payload is not valid base64url
- The decoded payload is not valid UTF-8

Errors are written to `stderr`.

## Development

Run the project locally:

```bash
cargo run --quiet
```

Format the code:

```bash
cargo fmt
```

Run tests:

```bash
cargo test
```

## Notes

This tool decodes the JWT payload only. It does not validate signatures, verify claims, or parse the JSON payload into structured output.
