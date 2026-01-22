# Glastocam Backend API

A Rust backend API built with Actix-Web for the Glastonbury Festival webcam viewer.

## Features

- **Health Check Endpoint**: Monitor API health status
- **Webcam Info API**: Get webcam configuration and metadata
- **Image Proxy**: Proxy webcam images through the backend (useful for CORS issues)
- **Static File Serving**: Serves the frontend directly
- **CORS Support**: Configured for cross-origin requests

## API Endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | `/api/v1/health` | Health check endpoint |
| GET | `/api/v1/info` | API information and available endpoints |
| GET | `/api/v1/webcam` | Get webcam configuration and metadata |
| GET | `/api/v1/webcam/url` | Get current webcam URL with cache-busting |
| GET | `/api/v1/webcam/proxy` | Proxy the webcam image through the backend |

## Prerequisites

- Rust 1.70 or later
- Cargo

## Getting Started

### Development

```bash
# Navigate to the backend directory
cd backend

# Build the project
cargo build

# Run the server
cargo run

# Run with logging
RUST_LOG=info cargo run
```

The server will start at `http://127.0.0.1:8080` by default.

### Production Build

```bash
cargo build --release
./target/release/glastocam-api
```

## Configuration

The API can be configured using environment variables:

| Variable | Default | Description |
|----------|---------|-------------|
| `HOST` | `127.0.0.1` | Host address to bind to |
| `PORT` | `8080` | Port to listen on |
| `WEBCAM_URL` | Panomax URL | Base URL for the webcam image |
| `REFRESH_INTERVAL_SECS` | `300` | Suggested refresh interval for clients |

## Docker

### Build

```bash
docker build -t glastocam-api ./backend
```

### Run

```bash
docker run -p 8080:8080 glastocam-api
```

## Example Responses

### Health Check
```json
{
  "status": "healthy",
  "timestamp": "2024-01-22T12:00:00Z",
  "version": "0.1.0"
}
```

### Webcam Info
```json
{
  "url": "https://panodata.panomax.com/cams/879/recent_full.jpg",
  "refresh_interval_secs": 300,
  "last_updated": "2024-01-22T12:00:00Z",
  "cache_bust_url": "https://panodata.panomax.com/cams/879/recent_full.jpg?ts=1705924800000"
}
```

## Project Structure

```
backend/
├── Cargo.toml           # Project dependencies
├── Dockerfile           # Docker configuration
├── README.md            # This file
└── src/
    ├── main.rs          # Application entry point
    ├── config.rs        # Configuration management
    ├── models.rs        # Data models/DTOs
    └── handlers/
        ├── mod.rs       # Route configuration
        ├── health.rs    # Health check handlers
        └── webcam.rs    # Webcam-related handlers
```

## License

Love the farm, leave no trace 💚

