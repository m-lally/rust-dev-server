#!/bin/bash

set -e

echo "🔨 Building optimized release binary..."
cargo build --release

echo ""
echo "✅ Build complete!"
echo "📦 Binary location: ./target/release/rust-dev-server"
echo "📊 Binary size:"
ls -lh ./target/release/rust-dev-server | awk '{print $5, $9}'
echo ""
echo "🚀 To run in production mode:"
echo "   ENVIRONMENT=production ./target/release/rust-dev-server"
