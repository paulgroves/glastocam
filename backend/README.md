# API Server

A simple Rust Actix-web backend API server with a hello world endpoint.

## Building

```bash
cargo build
```

## Running

```bash
cargo run
```

The server will start on `http://127.0.0.1:8080`

## Endpoints

- `GET /` - Returns a hello world message
- `GET /api/hello` - Returns a hello world message

## Example Response

```json
{
  "message": "Hello, World!",
  "status": "success"
}
```

