#!/bin/bash
# Rotate HMAC shared secret for IORA MCP authentication
# This script generates a new random secret and updates the environment

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# Generate new random secret (64 hex characters = 256 bits)
NEW_SECRET=$(openssl rand -hex 32)

echo "🔄 Rotating IORA MCP HMAC shared secret..."
echo "📁 Project root: $PROJECT_ROOT"

# Update .env file if it exists
ENV_FILE="$PROJECT_ROOT/.env"
if [ -f "$ENV_FILE" ]; then
    if grep -q "^CORAL_SHARED_SECRET=" "$ENV_FILE"; then
        # Replace existing secret
        sed -i.bak "s/^CORAL_SHARED_SECRET=.*/CORAL_SHARED_SECRET=$NEW_SECRET/" "$ENV_FILE"
        echo "✅ Updated CORAL_SHARED_SECRET in $ENV_FILE"
    else
        # Add new secret
        echo "CORAL_SHARED_SECRET=$NEW_SECRET" >> "$ENV_FILE"
        echo "✅ Added CORAL_SHARED_SECRET to $ENV_FILE"
    fi
else
    echo "⚠️  No .env file found. Creating one..."
    echo "CORAL_SHARED_SECRET=$NEW_SECRET" > "$ENV_FILE"
    echo "✅ Created $ENV_FILE with new secret"
fi

# Update MCP .env.example if it exists
MCP_ENV_EXAMPLE="$PROJECT_ROOT/mcp/.env.example"
if [ -f "$MCP_ENV_EXAMPLE" ]; then
    sed -i.bak "s/^CORAL_SHARED_SECRET=.*/CORAL_SHARED_SECRET=$NEW_SECRET/" "$MCP_ENV_EXAMPLE"
    echo "✅ Updated CORAL_SHARED_SECRET in $MCP_ENV_EXAMPLE"
fi

echo ""
echo "🔐 New HMAC Shared Secret: $NEW_SECRET"
echo ""
echo "⚠️  IMPORTANT SECURITY NOTES:"
echo "   • Store this secret securely (password manager, secret management system)"
echo "   • Never commit secrets to version control"
echo "   • Rotate secrets regularly (monthly recommended)"
echo "   • Update all client applications with new secret"
echo ""
echo "🔄 Restart MCP server to use new secret:"
echo "   cd $PROJECT_ROOT && make run"
echo ""
echo "🧪 Test with new secret:"
echo "   export CORAL_SHARED_SECRET=$NEW_SECRET"
echo "   make demo"
