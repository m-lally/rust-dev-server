#!/bin/bash

# Install cargo-watch if not already installed
if ! command -v cargo-watch &> /dev/null; then
    echo "📦 Installing cargo-watch..."
    cargo install cargo-watch
fi

echo "🚀 Starting development server with auto-reload..."
echo "💡 Server will restart automatically when you save Rust files"
echo "🌐 Visit http://localhost:3000"
echo ""
echo "Press Ctrl+C to stop"
echo ""

cargo watch -c -x run
