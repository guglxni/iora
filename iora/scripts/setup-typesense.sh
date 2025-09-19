#!/bin/bash

# Typesense Setup Script for I.O.R.A. Project
# This script sets up self-hosted Typesense via Docker

set -e

echo "🔍 Setting up self-hosted Typesense for I.O.R.A. project..."

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Check if Docker is installed
if ! command_exists docker; then
    echo "❌ Docker is not installed. Please install Docker first:"
    echo "   • macOS: https://docs.docker.com/docker-for-mac/install/"
    echo "   • Linux: https://docs.docker.com/engine/install/"
    echo "   • Windows: https://docs.docker.com/docker-for-windows/install/"
    exit 1
fi

# Check if Docker Compose is available
if ! command_exists docker-compose && ! docker compose version &> /dev/null; then
    echo "❌ Docker Compose is not available. Please install Docker Compose."
    exit 1
fi

# Create data directory for Typesense persistence
echo "📁 Creating Typesense data directory..."
mkdir -p ./assets/data

# Start Typesense service
echo "🐳 Starting Typesense service..."
if command_exists docker-compose; then
    docker-compose up -d typesense
else
    docker compose up -d typesense
fi

# Wait for Typesense to be ready
echo "⏳ Waiting for Typesense to start..."
sleep 10

# Test Typesense connection
echo "🔍 Testing Typesense connection..."
TYPESENSE_RESPONSE=$(curl -s -w "%{http_code}" -o /dev/null \
    -H "X-TYPESENSE-API-KEY: abc123xyz789" \
    "http://localhost:8108/health")

if [ "$TYPESENSE_RESPONSE" = "200" ]; then
    echo "✅ Typesense is running successfully!"
    echo "🌐 Typesense Dashboard: http://localhost:8108"
    echo "🔑 API Key: abc123xyz789"
else
    echo "❌ Typesense connection failed. HTTP status: $TYPESENSE_RESPONSE"
    echo "📋 Checking container status..."
    if command_exists docker-compose; then
        docker-compose ps typesense
        docker-compose logs typesense
    else
        docker compose ps typesense
        docker compose logs typesense
    fi
    exit 1
fi

# Create a sample collection for testing
echo "📚 Creating sample collection for testing..."
curl -X POST \
    -H "Content-Type: application/json" \
    -H "X-TYPESENSE-API-KEY: abc123xyz789" \
    "http://localhost:8108/collections" \
    -d '{
        "name": "test_collection",
        "fields": [
            {"name": "id", "type": "string"},
            {"name": "title", "type": "string"},
            {"name": "content", "type": "string"}
        ]
    }'

echo ""
echo "✅ Typesense setup complete!"
echo ""
echo "🔧 Service Information:"
echo "  • Dashboard: http://localhost:8108"
echo "  • API Endpoint: http://localhost:8108"
echo "  • API Key: abc123xyz789"
echo "  • Data Directory: ./assets/data"
echo ""
echo "🛠️ Management Commands:"
echo "  • Start: docker-compose up -d typesense"
echo "  • Stop: docker-compose down"
echo "  • Logs: docker-compose logs -f typesense"
echo "  • Restart: docker-compose restart typesense"
echo ""
echo "📚 Typesense Documentation:"
echo "  • API Reference: https://typesense.org/docs/latest/api/"
echo "  • Docker Setup: https://typesense.org/docs/latest/guide/install-typesense.html#docker"
