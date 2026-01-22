# Glastocam

Fullscreen version of Glasto webcam - https://glastocam.foobarlol.lol

Love the farm, leave no trace 💚

## Project Structure

- `index.html` - Frontend webcam viewer
- `backend/` - Rust/Actix-web API server
- `2016-General-Site-Plan-v3.pdf` - Festival site plan

## Backend API

The project includes a Rust backend built with Actix-web. See [backend/README.md](backend/README.md) for details.

### Quick Start

```bash
cd backend
cargo run
```

The server will start at `http://localhost:8080` and serve both the API and the frontend.

### API Endpoints

- `GET /api/health` - Health check
- `GET /api/cameras` - List cameras
- `GET /api/cameras/{id}/url` - Get camera URL with cache-busting
- `GET /api/settings` - Get settings
- `PUT /api/settings` - Update settings

