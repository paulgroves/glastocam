# Glastocam

Fullscreen version of Glasto webcam - https://glastocam.foobarlol.lol

Love the farm, leave no trace 💚

## Backend API

This project includes a Rust backend API built with actix-web.

### Building

```bash
cargo build --release
```

### Running

```bash
cargo run
```

The API server will start on `http://127.0.0.1:8080`

### API Endpoints

#### Health Check

- **GET** `/health` - Returns the health status of the API

Response (200 OK):
```json
{
  "status": "healthy"
}
```
