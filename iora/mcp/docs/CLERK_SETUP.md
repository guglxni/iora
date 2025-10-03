# Clerk Setup Guide for IORA

Quick start guide to set up Clerk authentication for IORA.

## Step 1: Create Clerk Account

1. Go to **[clerk.com](https://clerk.com)**
2. Click "Start Building for Free"
3. Sign up with GitHub or email
4. Verify your email

## Step 2: Create Application

1. In Clerk Dashboard, click **"Create Application"**
2. Name: `IORA` (or your preferred name)
3. Select authentication options:
   - ✅ Email
   - ✅ Google (optional)
   - ✅ GitHub (optional)
4. Click **"Create Application"**

## Step 3: Get API Keys

1. Go to **API Keys** in sidebar
2. Copy your keys:
   - **Publishable Key**: Starts with `pk_test_...` or `pk_live_...`
   - **Secret Key**: Starts with `sk_test_...` or `sk_live_...`

## Step 4: Configure Environment

Add to `/Volumes/MacExt/desktop-backup-sep-24/iora/iora/mcp/.env`:

```bash
# Clerk Authentication
CLERK_PUBLISHABLE_KEY=pk_test_YOUR_KEY_HERE
CLERK_SECRET_KEY=sk_test_YOUR_KEY_HERE
CLERK_WEBHOOK_SECRET=whsec_YOUR_WEBHOOK_SECRET
```

**Note**: Create `.env` from `api-keys-template.env` if it doesn't exist:
```bash
cd iora/mcp
cp api-keys-template.env .env
# Then edit .env with your actual keys
```

## Step 5: Enable Organizations (Optional)

Organizations allow multi-tenant features (teams, billing per org).

1. In Clerk Dashboard, go to **Settings** → **Organizations**
2. Toggle **"Enable Organizations"**
3. Configure:
   - **Roles**: Admin, Editor, Viewer
   - **Permissions**: Manage members, billing, etc.
4. Save changes

## Step 6: Configure Webhooks (For Production)

Webhooks notify your server of user events (sign-ups, deletions, tier changes).

1. Go to **Webhooks** in Clerk Dashboard
2. Click **"Add Endpoint"**
3. URL: `https://your-domain.com/webhooks/clerk`
4. Events to subscribe:
   - `user.created`
   - `user.updated`
   - `user.deleted`
   - `organization.created`
   - `organization.updated`
5. Copy **Signing Secret** (starts with `whsec_...`)
6. Add to `.env` as `CLERK_WEBHOOK_SECRET`

## Step 7: Test Setup

```bash
# Start IORA MCP server
cd iora/mcp
npm run dev

# In another terminal, test user endpoint (will fail without auth - expected)
curl http://localhost:7070/user/profile

# Expected response:
# {"ok":false,"error":"Unauthorized - No session token provided"}

# If you see this, Clerk is configured correctly!
```

## Step 8: Test with Frontend (Optional)

If you want to test with a UI:

```bash
# Install Clerk React SDK
cd demo  # or your frontend directory
npm install @clerk/nextjs

# Add Clerk provider to your app
# See: https://clerk.com/docs/quickstarts/nextjs
```

## Pricing Tiers

Clerk offers generous free tier:

| Tier | Price | MAUs* | Features |
|------|-------|-------|----------|
| **Free** | $0/month | 10,000 | All auth features, organizations |
| **Pro** | $25/month | 10,000 + $0.02/user | Advanced features, priority support |
| **Enterprise** | Custom | Unlimited | SLA, custom contracts |

*MAUs = Monthly Active Users

## Troubleshooting

### "Invalid secret key"
- Double-check you copied the **Secret Key** (not Publishable Key)
- Ensure no extra spaces/newlines in `.env`
- Restart server after updating `.env`

### "Session verification failed"
- Check `CLERK_SECRET_KEY` is set correctly
- Verify session token is being sent in request
- Check Clerk Dashboard for any API issues

### "Organizations not available"
- Enable Organizations in Clerk Dashboard (Settings → Organizations)
- May need to upgrade to Pro plan for advanced org features

## Next Steps

✅ **Clerk is now configured!**

1. **Create test users**: Use Clerk Dashboard → Users → Create User
2. **Test API keys**: Call `/user/api-keys` to generate keys
3. **Integrate billing**: See `docs/BILLING.md` for Stripe setup
4. **Build dashboard**: See `docs/ADMIN_DASHBOARD.md`

## Useful Resources

- [Clerk Documentation](https://clerk.com/docs)
- [Clerk Node.js SDK](https://clerk.com/docs/references/nodejs/overview)
- [Clerk + Express Guide](https://clerk.com/docs/references/nodejs/express)
- [Organizations Guide](https://clerk.com/docs/organizations/overview)

---

**Questions?** Check [AUTHENTICATION.md](./AUTHENTICATION.md) for detailed API reference.

