# Glastocam

Fullscreen version of Glasto webcam - https://glastocam.foobarlol.lol

Love the farm, leave no trace 💚

## Backend API

A Rust actix-web backend API for Glastocam.

### Prerequisites

- Rust (install via [rustup](https://rustup.rs/))

### Running the API

```bash
cargo run
```

The server will start at http://127.0.0.1:8080

### Endpoints

- `GET /` - Hello World endpoint
- `GET /health` - Health check endpoint

### Building

```bash
cargo build --release
```

