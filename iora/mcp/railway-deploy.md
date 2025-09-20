# 🚀 Railway Deployment Guide for IORA MCP Server

## Step 1: Manual Railway Setup

1. **Login to Railway**:
   ```bash
   railway login
   ```
   This will open a browser window for authentication.

2. **Initialize Railway Project**:
   ```bash
   railway init
   ```
   Choose "Create new project" and name it `iora-mcp-server`.

3. **Deploy the Application**:
   ```bash
   railway up
   ```

## Step 2: Set Environment Variables in Railway Dashboard

Go to your Railway project dashboard and add these environment variables:

### AI Provider Keys
```
GEMINI_API_KEY = AIzaSyArBC8Ic8CrTWxqiuBGYPnJV2NaXP2vFrY
OPENAI_API_KEY = sk-proj-1234567890abcdefghijklmnopqrstuvwx
MISTRAL_API_KEY = IZ2b3OsuTXBTDHzgLwSUtMuBFK7o4U5K
AIMLAPI_KEY = sk-aiml-prod-abcdefghijklmnopqrst
```

### Cryptocurrency API Keys
```
COINGECKO_API_KEY = CG-eFaWUkU2eVW3uYHL7aFXDDC7
COINMARKETCAP_API_KEY = e411ef8f-e03b-48e2-81c1-087151a9fa04
CRYPTOCOMPARE_API_KEY = 324e5a2144c5478f59a78767f75465f4d04a3c922bfee60691c823cc45bc49a4
COINPAPRIKA_API_KEY = sk-coinpaprika-prod-abcdefghijklmnopqrst
```

### Crossmint Production Configuration
```
CROSSMINT_PROJECT_ID = a87e9abb-4345-4626-8d2c-06f641a35c11
CROSSMINT_SERVER_SECRET = sk_production_9oKEK9drjW54HLZjfv74yQ27CRNutqUCJhxNHZT2mLJHN2k23vZWAELbCfAGUN95xFXPdPJFjEJgps4idKDjBqfbh5n9zoHLYkECNyiLzSFhyfN8YdzfyQhCjhXbX1rDS14cZ3K5ur6y7ZHLqEcn72NjKdnjhad4kJTjmYGHDUzALNx6Wxafq9Hy2SEAAsexqVd8DSdMpRuhXQEofytyXvTK
CROSSMINT_CLIENT_KEY = ck_production_9oKEK9drjW54HLZjfv74yQ27CRNutqUCJhxNHZT2mLJHN2k23vZWAELbCfAGUN95xFXPdPJFjEJgps4idKDjBqfbh5n9zoHLYkECNyiLzSFhyfN8YdzfyQhCjhXbX1rDS14cZ3K5ur6y7ZHLqEcn72NjKdnjhad4kJTjmYGHDUzALNx6Wxafq9Hy2SEAAsexqVd8DSdMpRuhXQEofytyXvTK
CROSSMINT_ENV = production
CROSSMINT_RECIPIENT = 9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM
```

### Server Configuration
```
CORAL_SHARED_SECRET = iora-production-secret-2025
CORAL_REGISTRY_AUTO_REGISTER = false
PORT = 8000
```

### Blockchain Configuration
```
SOLANA_RPC_URL = https://api.devnet.solana.com
SOLANA_WALLET_PATH = /app/wallets/devnet-wallet.json
```

### Database Configuration
```
TYPESENSE_API_KEY = iora-production-typesense-key-2025
TYPESENSE_URL = http://localhost:8108
```

### Model Configuration
```
MISTRAL_MODEL = mistral-medium
IORA_BIN = /app/iora
```

## Step 3: After Deployment

1. **Get your Railway URL** (something like `https://iora-mcp-server-production.up.railway.app`)
2. **Test the health endpoint**: `https://your-url.railway.app/tools/health`
3. **Update Vercel environment variables** with your new Railway URL

## Step 4: Update Vercel

In your Vercel dashboard, update:
```
IORA_SERVER_URL = https://your-railway-url.railway.app
IORA_SHARED_SECRET = iora-production-secret-2025
```
