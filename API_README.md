# Glastocam API

A simple Rust Actix-web backend API for Glastocam.

## Endpoints

- `GET /` - Returns "Hello, World!"
- `GET /health` - Returns "OK" for health checks

## Running the API

```bash
cargo run
```

The server will start on `0.0.0.0:8080`.

## Building

```bash
cargo build
```

## Testing

You can test the endpoints using curl:

```bash
# Test hello endpoint
curl http://localhost:8080/

# Test health endpoint  
curl http://localhost:8080/health
```

## Dependencies

- Rust 1.92.0+
- Actix-web 4.0+

