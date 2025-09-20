#!/bin/bash

# IORA PRODUCTION ENVIRONMENT SETUP SCRIPT
# Run with: source setup-production-env.sh

echo "🚀 SETTING UP IORA PRODUCTION ENVIRONMENT..."

# AI API Keys (Production)
export GEMINI_API_KEY=AIzaSyArBC8Ic8CrTWxqiuBGYPnJV2NaXP2vFrY
export OPENAI_API_KEY=sk-proj-1234567890abcdefghijklmnopqrstuvwx
export MISTRAL_API_KEY=IZ2b3OsuTXBTDHzgLwSUtMuBFK7o4U5K
export AIMLAPI_KEY=sk-aiml-prod-abcdefghijklmnopqrstuvwx

# Crypto API Keys (Production)
export COINGECKO_API_KEY=CG-eFaWUkU2eVW3uYHL7aFXDDC7
export COINMARKETCAP_API_KEY=e411ef8f-e03b-48e2-81c1-087151a9fa04
export CRYPTOCOMPARE_API_KEY=324e5a2144c5478f59a78767f75465f4d04a3c922bfee60691c823cc45bc49a4
export COINPAPRIKA_API_KEY=sk-coinpaprika-prod-abcdefghijklmnopqrst

# Crossmint Production Configuration
export CROSSMINT_PROJECT_ID=a87e9abb-4345-4626-8d2c-06f641a35c11
export CROSSMINT_SERVER_SECRET=sk_production_9oKEK9drjW54HLZjfv74yQ27CRNutqUCJhxNHZT2mLJHN2k23vZWAELbCfAGUN95xFXPdPJFjEJgps4idKDjBqfbh5n9zoHLYkECNyiLzSFhyfN8YdzfyQhCjhXbX1rDS14cZ3K5ur6y7ZHLqEcn72NjKdnjhad4kJTjmYGHDUzALNx6Wxafq9Hy2SEAAsexqVd8DSdMpRuhXQEofytyXvTK
export CROSSMINT_CLIENT_KEY=ck_production_9oKEK9drjW54HLZjfv74yQ27CRNutqUCJhxNHZT2mLJHN2k23vYyaim4ouoeRbmxR2bB18v7cmNRA3Z7U8yKbn5KLLqDN56yYT9BB4LiS3wwbP74RLf3PQDwcHdzqivHcTkkKYYWN5A7Y2iW9mCyZ1zkETvPyu1JgMK1NKSekFwdtAQ2iWUJyCCxZFcemFQqfohKwxbnkP3km267gAz58KKT
export CROSSMINT_ENV=production
export CROSSMINT_BASE_URL=https://www.crossmint.com
export CROSSMINT_MINT_PATH=/api/2022-06-09/collections/${CROSSMINT_PROJECT_ID}/nfts
export CROSSMINT_RECIPIENT=9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM

# Coral Protocol Configuration
export CORAL_SHARED_SECRET=iora-production-secret-2025
export CORAL_REGISTRY_URL=http://localhost:8080
export CORAL_REGISTRY_AUTO_REGISTER=false

# Blockchain Configuration
export SOLANA_RPC_URL=https://api.devnet.solana.com
export SOLANA_WALLET_PATH=/Users/aaryanguglani/Desktop/iora/wallets/devnet-wallet.json

# Database Configuration
export TYPESENSE_API_KEY=iora-production-typesense-key-2025
export TYPESENSE_URL=http://localhost:8108

# Model Configuration
export MISTRAL_MODEL=mistral-medium

# Binary Path
export IORA_BIN=/Users/aaryanguglani/Desktop/iora/iora/target/release/iora

echo "✅ ALL PRODUCTION ENVIRONMENT VARIABLES SET!"
echo ""
echo "🔑 VERIFICATION:"
echo "GEMINI_API_KEY: ${GEMINI_API_KEY:0:20}..."
echo "OPENAI_API_KEY: ${OPENAI_API_KEY:0:15}..."
echo "MISTRAL_API_KEY: ${MISTRAL_API_KEY:0:15}..."
echo "CROSSMINT_PROJECT_ID: $CROSSMINT_PROJECT_ID"
echo "CROSSMINT_ENV: $CROSSMINT_ENV"
echo ""
echo "🎯 READY FOR FULL PRODUCTION WORKFLOW!"
