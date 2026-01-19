# Glastocam

Fullscreen version of Glasto webcam - https://glastocam.foobarlol.lol

Love the farm, leave no trace 💚

## Backend API

The backend is a Rust actix-web application located in the `backend/` directory.

### Prerequisites

- Rust (install via [rustup](https://rustup.rs/))

### Running the Backend

```bash
cd backend
cargo run
```

The server will start at `http://127.0.0.1:8080`

### API Endpoints

- `GET /` - Returns "Hello, World!"
- `GET /health` - Returns health status JSON

