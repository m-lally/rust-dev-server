# Rust Development Server

A production-ready HTTP server built with Rust, featuring automatic reload, structured logging, and best practices for local development and production deployment.

## Features

✅ **Production-Ready**

- Graceful shutdown handling (SIGTERM/SIGINT)
- Structured logging with tracing
- Request ID middleware for traceability
- Health check endpoints
- CORS support
- GZIP compression
- Proper error handling

✅ **Developer Experience**

- Automatic reload on code changes
- Environment-based configuration
- Static file serving
- REST API example endpoints

## Quick Start

### Prerequisites

- Rust 1.70+ (install via [rustup](https://rustup.rs/))
- For auto-reload: `cargo install cargo-watch`

### Installation

```bash
cd /Users/scarecro/Projects/rust-dev-server

# Install dependencies
cargo build

# Run with auto-reload (recommended for development)
cargo watch -x run

# Or run directly
cargo run

# Production build
cargo build --release
./target/release/rust-dev-server
```

The server will start on `http://localhost:3000`

## Configuration

Configure via environment variables or `.env` file:

```bash
PORT=3000                    # Server port
STATIC_DIR=./public         # Static files directory
ENVIRONMENT=development     # development/production
LOG_LEVEL=debug            # trace/debug/info/warn/error
```

## API Endpoints

| Method | Path          | Description                       |
| ------ | ------------- | --------------------------------- |
| GET    | `/api/health` | Health check with version info    |
| GET    | `/api/items`  | List all items                    |
| POST   | `/api/items`  | Create new item                   |
| POST   | `/api/echo`   | Echo back JSON payload            |
| GET    | `/`           | Serve static files from `public/` |

### Examples

```bash
# Health check
curl http://localhost:3000/api/health

# List items
curl http://localhost:3000/api/items

# Create item
curl -X POST http://localhost:3000/api/items \
  -H "Content-Type: application/json" \
  -d '{"name":"New Item","description":"Item description"}'

# Echo
curl -X POST http://localhost:3000/api/echo \
  -H "Content-Type: application/json" \
  -d '{"message":"Hello, Rust!"}'
```

## Auto-Reload Development

Install cargo-watch:

```bash
cargo install cargo-watch
```

Run with auto-reload:

```bash
# Reload on any Rust file change
cargo watch -x run

# Clear screen on reload
cargo watch -c -x run

# Run with specific features
cargo watch -x 'run --features development'
```

## Project Structure

```
rust-dev-server/
├── src/
│   ├── main.rs              # Application entry point
│   ├── config.rs            # Configuration management
│   ├── handlers.rs          # HTTP request handlers
│   └── middleware/
│       ├── mod.rs
│       └── request_id.rs    # Request ID middleware
├── public/                  # Static files
│   └── index.html
├── Cargo.toml
├── .env                     # Environment variables
└── README.md
```

## Production Deployment

### Build Optimized Binary

```bash
cargo build --release
# Binary at: ./target/release/rust-dev-server
```

The release profile includes:

- LTO (Link-Time Optimization)
- Single codegen unit
- Stripped symbols
- Level 3 optimization

### Docker Deployment (Optional)

Create a `Dockerfile`:

```dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/rust-dev-server /usr/local/bin/
COPY --from=builder /app/public /app/public
ENV STATIC_DIR=/app/public
ENV PORT=3000
EXPOSE 3000
CMD ["rust-dev-server"]
```

### systemd Service (Linux)

Create `/etc/systemd/system/rust-dev-server.service`:

```ini
[Unit]
Description=Rust Development Server
After=network.target

[Service]
Type=simple
User=www-data
WorkingDirectory=/opt/rust-dev-server
Environment="PORT=3000"
Environment="ENVIRONMENT=production"
ExecStart=/opt/rust-dev-server/rust-dev-server
Restart=on-failure

[Install]
WantedBy=multi-user.target
```

## Testing

```bash
# Run tests
cargo test

# Run with coverage
cargo tarpaulin --out Html
```

## Performance Considerations

- **Connection pooling**: Add a database connection pool if using a DB
- **Rate limiting**: Implement rate limiting for production
- **Caching**: Add Redis/in-memory caching for frequently accessed data
- **Load balancing**: Run multiple instances behind nginx/HAProxy

## Security Checklist

- [ ] Enable HTTPS in production (use reverse proxy like nginx)
- [ ] Implement rate limiting
- [ ] Add authentication/authorization
- [ ] Validate all inputs
- [ ] Set security headers (CSP, HSTS, etc.)
- [ ] Regular dependency updates (`cargo audit`)
- [ ] Environment-specific secrets management

## Monitoring

The server includes:

- Structured logging (JSON in production)
- Request IDs for tracing
- Health check endpoint

Consider adding:

- Prometheus metrics
- OpenTelemetry integration
- Error tracking (Sentry)

## License

MIT
