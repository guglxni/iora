# 🚀 IORA Railway Deployment Instructions

## Quick Deploy (2 Commands Only!)

**1. Login to Railway:**
```bash
cd /Users/aaryanguglani/Desktop/iora/iora/mcp
railway login
```
*(This opens your browser - just click "Login with GitHub" or your preferred method)*

**2. Run the deployment script:**
```bash
./deploy-to-railway.sh
```

That's it! The script handles everything else:
- ✅ Creates Railway project
- ✅ Sets all 20+ environment variables
- ✅ Deploys your IORA MCP server
- ✅ Gets your deployment URL
- ✅ Tests the health endpoint

## What You'll Get

After deployment, you'll see:
```
✅ DEPLOYMENT SUCCESSFUL!
🌐 Your IORA MCP Server is live at: https://iora-mcp-server-production.up.railway.app
```

## Next Steps

1. **Copy your Railway URL**
2. **Update Vercel environment variables:**
   - `IORA_SERVER_URL` = your Railway URL
   - `IORA_SHARED_SECRET` = `iora-production-secret-2025`
3. **Test your live demo!**

## Manual Alternative (if script fails)

If the script doesn't work, run these commands one by one:

```bash
# Initialize project
railway init --name "iora-mcp-server"

# Set environment variables (copy-paste from railway-env-vars.txt)
railway variables set GEMINI_API_KEY="AIzaSyArBC8Ic8CrTWxqiuBGYPnJV2NaXP2vFrY"
# ... (all other variables)

# Deploy
railway up
```

## 🎯 Ready to Deploy!

Run the two commands above and your IORA MCP server will be live on Railway!
