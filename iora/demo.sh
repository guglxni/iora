#!/bin/bash
# IORA MCP Demo Script
# Runs complete end-to-end flow: health → price → analysis → oracle feed → receipt mint

set -e

# Configuration
PORT=${PORT:-7070}
SECRET=${CORAL_SHARED_SECRET:?Set CORAL_SHARED_SECRET environment variable}

echo "🚀 IORA MCP End-to-End Demo"
echo "================================"
echo

# Function to generate HMAC signature
generate_sig() {
    local body="$1"
    echo -n "$body" | openssl dgst -sha256 -hmac "$SECRET" | awk '{print $2}'
}

# 1. Health Check
echo "1. 🏥 Health Check"
echo "-------------------"
curl -s http://localhost:$PORT/tools/health | jq '.'
echo -e "\n"

# 2. Get Price
echo "2. 💰 Get Price (BTC)"
echo "----------------------"
PRICE_BODY='{"symbol":"BTC"}'
PRICE_SIG=$(generate_sig "$PRICE_BODY")
PRICE_RESPONSE=$(curl -s -H "x-iora-signature: $PRICE_SIG" -H "content-type: application/json" \
    -d "$PRICE_BODY" http://localhost:$PORT/tools/get_price)

echo "$PRICE_RESPONSE" | jq '.'
PRICE=$(echo "$PRICE_RESPONSE" | jq -r '.data.price // 0')
echo -e "\n"

# 3. Analyze Market
echo "3. 📊 Analyze Market (BTC, 1d, Mistral)"
echo "----------------------------------------"
ANALYSIS_BODY='{"symbol":"BTC","horizon":"1d","provider":"mistral"}'
ANALYSIS_SIG=$(generate_sig "$ANALYSIS_BODY")
curl -s -H "x-iora-signature: $ANALYSIS_SIG" -H "content-type: application/json" \
    -d "$ANALYSIS_BODY" http://localhost:$PORT/tools/analyze_market | jq '.'
echo -e "\n"

# 4. Feed Oracle
echo "4. 🔗 Feed Oracle (BTC)"
echo "------------------------"
ORACLE_BODY='{"symbol":"BTC"}'
ORACLE_SIG=$(generate_sig "$ORACLE_BODY")
ORACLE_RESPONSE=$(curl -s -H "x-iora-signature: $ORACLE_SIG" -H "content-type: application/json" \
    -d "$ORACLE_BODY" http://localhost:$PORT/tools/feed_oracle)

echo "$ORACLE_RESPONSE" | jq '.'
TX=$(echo "$ORACLE_RESPONSE" | jq -r '.data.tx // ""')
echo -e "\n"

# 5. Mint Receipt (if Crossmint configured)
if [ -n "$CROSSMINT_API_KEY" ] && [ -n "$CROSSMINT_PROJECT_ID" ] && [ -n "$TX" ]; then
    echo "5. 🎨 Mint Receipt NFT"
    echo "-----------------------"
    RECEIPT_BODY=$(jq -n \
        --arg symbol "BTC" \
        --arg price "$PRICE" \
        --arg tx "$TX" \
        --arg model "mistral" \
        --argjson ts "$(date +%s)" \
        '{symbol: $symbol, price: ($price | tonumber), tx: $tx, model: $model, ts: $ts}')

    RECEIPT_SIG=$(generate_sig "$RECEIPT_BODY")
    RECEIPT_RESPONSE=$(curl -s -H "x-iora-signature: $RECEIPT_SIG" -H "content-type: application/json" \
        -d "$RECEIPT_BODY" http://localhost:$PORT/receipt)

    if echo "$RECEIPT_RESPONSE" | jq -e '.ok' > /dev/null 2>&1; then
        echo "$RECEIPT_RESPONSE" | jq '.'
        RECEIPT_ID=$(echo "$RECEIPT_RESPONSE" | jq -r '.id // ""')
        if [ -n "$RECEIPT_ID" ]; then
            echo -e "\n✅ Receipt minted! ID: $RECEIPT_ID"
        fi
    else
        echo "❌ Receipt minting failed or not configured"
        echo "$RECEIPT_RESPONSE" | jq '.' 2>/dev/null || echo "$RECEIPT_RESPONSE"
    fi
else
    echo "5. 🎨 Receipt Minting Skipped"
    echo "------------------------------"
    echo "❌ Crossmint not configured (set CROSSMINT_API_KEY and CROSSMINT_PROJECT_ID)"
fi

echo -e "\n🎉 Demo Complete!"
echo "=================="
echo "IORA successfully demonstrated:"
echo "• Multi-API price fetching"
echo "• Multi-provider LLM analysis"
echo "• Solana oracle feed integration"
echo "• Optional Crossmint receipt minting"
echo
echo "Check the logs above for detailed execution times and results."
