# Glastocam

Fullscreen version of Glasto webcam - https://glastocam.foobarlol.lol

Love the farm, leave no trace 💚

## API Backend

A Rust-based backend API built with Actix-web.

### Getting Started

#### Prerequisites

- Rust 1.56+ (install from https://rustup.rs/)

#### Building

```bash
cargo build
```

#### Running

```bash
cargo run
```

The server will start on `http://127.0.0.1:8080`

### Endpoints

#### GET /

Returns a simple hello world JSON response.

**Response:**
```json
{
  "message": "Hello, World!"
}
```

### Development

To run with automatic reloading during development, install `cargo-watch`:

```bash
cargo install cargo-watch
cargo watch -x run
```

### Building for Production

```bash
cargo build --release
```

The optimized binary will be available at `target/release/api`

