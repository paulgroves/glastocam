# Glastocam API

A simple Rust actix-web backend API for the Glastocam project.

## Prerequisites

- Rust (latest stable version)

## Running the Server

```bash
cd backend
cargo run
```

The server will start at `http://127.0.0.1:8080`.

## API Endpoints

| Method | Endpoint  | Description           |
|--------|-----------|----------------------|
| GET    | `/`       | Returns "Hello, World!" |
| GET    | `/health` | Health check endpoint   |

## Development

### Build

```bash
cargo build
```

### Run in Release Mode

```bash
cargo run --release
```
