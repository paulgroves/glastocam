# Glastocam API

A backend API built with Rust and Actix-web for the Glastonbury Festival webcam viewer.

## Features

- RESTful API endpoints for camera management
- Health check endpoint
- Configurable refresh intervals
- CORS support for frontend integration
- Static file serving for the frontend

## API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/health` | Health check |
| GET | `/api/info` | API information |
| GET | `/api/cameras` | List all cameras |
| GET | `/api/cameras/{id}` | Get camera by ID |
| GET | `/api/cameras/{id}/url` | Get camera URL with cache-busting timestamp |
| GET | `/api/settings` | Get current settings |
| PUT | `/api/settings` | Update settings (refresh interval) |

## Running the Server

### Prerequisites

- Rust (1.70 or later)
- Cargo

### Development

```bash
# Build the project
cargo build

# Run the server
cargo run

# Run with logging
RUST_LOG=info cargo run
```

### Configuration

Environment variables:

- `BIND_ADDRESS`: Server bind address (default: `127.0.0.1`)
- `PORT`: Server port (default: `8080`)
- `RUST_LOG`: Log level (e.g., `info`, `debug`, `error`)

### Production Build

```bash
cargo build --release
./target/release/glastocam-api
```

## API Examples

### Get all cameras
```bash
curl http://localhost:8080/api/cameras
```

### Get camera URL with cache-busting
```bash
curl http://localhost:8080/api/cameras/1/url
```

### Update refresh interval
```bash
curl -X PUT http://localhost:8080/api/settings \
  -H "Content-Type: application/json" \
  -d '{"refresh_interval_ms": 60000}'
```

## License

MIT

