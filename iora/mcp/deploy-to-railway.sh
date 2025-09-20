#!/bin/bash

echo "🚀 IORA MCP Server - Railway Deployment Script"
echo "=============================================="

# Check if logged in to Railway
if ! railway whoami > /dev/null 2>&1; then
    echo "❌ Not logged into Railway. Please run: railway login"
    echo "   This will open your browser for authentication."
    echo ""
    echo "   After logging in, run this script again."
    exit 1
fi

echo "✅ Railway authentication verified"
echo ""

# Initialize Railway project
echo "🔧 Initializing Railway project..."
if [ ! -f "railway.toml" ]; then
    railway init --name "iora-mcp-server"
else
    echo "   Railway project already initialized"
fi

echo ""
echo "⚙️  Setting environment variables..."

# Set all environment variables
railway variables set GEMINI_API_KEY="AIzaSyArBC8Ic8CrTWxqiuBGYPnJV2NaXP2vFrY"
railway variables set OPENAI_API_KEY="sk-proj-1234567890abcdefghijklmnopqrstuvwx"
railway variables set MISTRAL_API_KEY="IZ2b3OsuTXBTDHzgLwSUtMuBFK7o4U5K"
railway variables set AIMLAPI_KEY="sk-aiml-prod-abcdefghijklmnopqrstuvwx"

railway variables set COINGECKO_API_KEY="CG-eFaWUkU2eVW3uYHL7aFXDDC7"
railway variables set COINMARKETCAP_API_KEY="e411ef8f-e03b-48e2-81c1-087151a9fa04"
railway variables set CRYPTOCOMPARE_API_KEY="324e5a2144c5478f59a78767f75465f4d04a3c922bfee60691c823cc45bc49a4"
railway variables set COINPAPRIKA_API_KEY="sk-coinpaprika-prod-abcdefghijklmnopqrst"

railway variables set CROSSMINT_PROJECT_ID="a87e9abb-4345-4626-8d2c-06f641a35c11"
railway variables set CROSSMINT_SERVER_SECRET="sk_production_9oKEK9drjW54HLZjfv74yQ27CRNutqUCJhxNHZT2mLJHN2k23vZWAELbCfAGUN95xFXPdPJFjEJgps4idKDjBqfbh5n9zoHLYkECNyiLzSFhyfN8YdzfyQhCjhXbX1rDS14cZ3K5ur6y7ZHLqEcn72NjKdnjhad4kJTjmYGHDUzALNx6Wxafq9Hy2SEAAsexqVd8DSdMpRuhXQEofytyXvTK"
railway variables set CROSSMINT_CLIENT_KEY="ck_production_9oKEK9drjW54HLZjfv74yQ27CRNutqUCJhxNHZT2mLJHN2k23vZWAELbCfAGUN95xFXPdPJFjEJgps4idKDjBqfbh5n9zoHLYkECNyiLzSFhyfN8YdzfyQhCjhXbX1rDS14cZ3K5ur6y7ZHLqEcn72NjKdnjhad4kJTjmYGHDUzALNx6Wxafq9Hy2SEAAsexqVd8DSdMpRuhXQEofytyXvTK"
railway variables set CROSSMINT_ENV="production"
railway variables set CROSSMINT_RECIPIENT="9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM"

railway variables set CORAL_SHARED_SECRET="iora-production-secret-2025"
railway variables set CORAL_REGISTRY_AUTO_REGISTER="false"

railway variables set SOLANA_RPC_URL="https://api.devnet.solana.com"
railway variables set SOLANA_WALLET_PATH="./wallets/devnet-wallet.json"

railway variables set TYPESENSE_API_KEY="iora-production-typesense-key-2025"
railway variables set TYPESENSE_URL="http://localhost:8108"

railway variables set MISTRAL_MODEL="mistral-medium"
railway variables set IORA_BIN="./iora"
railway variables set PORT="8000"

echo ""
echo "🚀 Deploying to Railway..."
railway up --detach

echo ""
echo "⏳ Waiting for deployment to complete..."
sleep 10

echo ""
echo "🔗 Getting deployment URL..."
RAILWAY_URL=$(railway status --json | jq -r '.deployments[0].url' 2>/dev/null || railway domain)

if [ -n "$RAILWAY_URL" ] && [ "$RAILWAY_URL" != "null" ]; then
    echo ""
    echo "✅ DEPLOYMENT SUCCESSFUL!"
    echo "🌐 Your IORA MCP Server is live at: $RAILWAY_URL"
    echo ""
    echo "🧪 Testing health endpoint..."
    curl -s "$RAILWAY_URL/tools/health" | jq . || echo "Health check will be available once deployment completes"
    echo ""
    echo "📝 Next steps:"
    echo "1. Update your Vercel environment variables:"
    echo "   IORA_SERVER_URL = $RAILWAY_URL"
    echo "   IORA_SHARED_SECRET = iora-production-secret-2025"
    echo ""
    echo "2. Test your frontend at: https://your-vercel-app.vercel.app"
else
    echo "⚠️  Deployment initiated. Check Railway dashboard for status."
    echo "   Run 'railway status' to check deployment progress"
fi

echo ""
echo "🎉 Railway deployment script completed!"
