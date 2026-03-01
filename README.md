# Glastocam

Fullscreen version of Glasto webcam - https://glastocam.foobarlol.lol

Love the farm, leave no trace 💚

## Backend API

A Rust Actix-web backend API server.

### Building

```bash
cargo build
```

### Running

```bash
cargo run
```

The server will start on `http://127.0.0.1:8080`

### Endpoints

#### GET /
Returns a simple hello world message.

**Response:**
```json
{
  "message": "Hello, World!"
}
```

### Dependencies

- **actix-web**: Web framework for Rust
- **tokio**: Async runtime
- **serde_json**: JSON serialization/deserialization
