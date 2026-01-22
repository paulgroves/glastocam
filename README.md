# Glastocam

Fullscreen version of Glasto webcam - https://glastocam.foobarlol.lol

Love the farm, leave no trace 💚

## Project Structure

- `index.html` - Frontend webcam viewer
- `2016-General-Site-Plan-v3.pdf` - Festival site plan
- `backend/` - Rust/Actix-Web backend API

## Backend API

The project includes a Rust backend API built with Actix-Web. See [backend/README.md](backend/README.md) for details.

### Quick Start

```bash
cd backend
cargo run
```

The API provides:
- Health check endpoints
- Webcam metadata and configuration
- Image proxy for CORS support
- Static file serving

