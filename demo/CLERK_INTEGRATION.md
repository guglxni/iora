# Clerk Integration - IORA Next.js Demo

This document describes the Clerk authentication integration in the IORA Next.js demo application following the **official App Router approach**.

## ✅ Implementation Overview

The integration follows Clerk's official Next.js App Router quickstart from [clerk.com/docs/quickstarts/nextjs](https://clerk.com/docs/quickstarts/nextjs).

### Files Added

1. **`middleware.ts`** - Clerk middleware using `clerkMiddleware()`
2. **`src/app/sign-in/[[...sign-in]]/page.tsx`** - Custom sign-in page
3. **`src/app/sign-up/[[...sign-up]]/page.tsx`** - Custom sign-up page
4. **`src/app/dashboard/page.tsx`** - Protected dashboard for authenticated users

### Files Modified

1. **`src/app/layout.tsx`** - Added `<ClerkProvider>` and auth UI components
2. **`src/app/page.tsx`** - Updated with conditional rendering based on auth state
3. **`package.json`** - Added `@clerk/nextjs` dependency

## 🔧 Setup Instructions

### 1. Environment Variables

Create a `.env.local` file in the demo directory with your Clerk keys:

```bash
# Clerk Authentication Keys
NEXT_PUBLIC_CLERK_PUBLISHABLE_KEY=pk_test_cGVhY2VmdWwtY3JpY2tldC03NS5jbGVyay5hY2NvdW50cy5kZXYk
CLERK_SECRET_KEY=sk_test_ItECHsuh9yuhGSdCQ0brbGuiv8CqQwh2Men7GGo3aD

# Clerk Redirect URLs
NEXT_PUBLIC_CLERK_SIGN_IN_URL=/sign-in
NEXT_PUBLIC_CLERK_SIGN_UP_URL=/sign-up
NEXT_PUBLIC_CLERK_SIGN_IN_FALLBACK_REDIRECT_URL=/
NEXT_PUBLIC_CLERK_SIGN_UP_FALLBACK_REDIRECT_URL=/
```

**Note:** `.env.local` is in `.gitignore` and should never be committed.

### 2. Install Dependencies

```bash
cd demo
npm install
```

Dependencies installed:
- `@clerk/nextjs@latest` - Clerk Next.js SDK

### 3. Run Development Server

```bash
npm run dev
```

Visit `http://localhost:3000`

## 📁 File Structure

```
demo/
├── middleware.ts                       # Clerk middleware
├── .env.local                          # Environment variables (not committed)
├── src/
│   └── app/
│       ├── layout.tsx                  # Root layout with ClerkProvider
│       ├── page.tsx                    # Landing page
│       ├── dashboard/
│       │   └── page.tsx               # Protected dashboard
│       ├── sign-in/
│       │   └── [[...sign-in]]/
│       │       └── page.tsx           # Sign-in page
│       └── sign-up/
│           └── [[...sign-up]]/
│               └── page.tsx           # Sign-up page
```

## 🎯 Key Implementation Details

### 1. Middleware (`middleware.ts`)

Using the **latest** `clerkMiddleware()` from `@clerk/nextjs/server`:

```typescript
import { clerkMiddleware } from '@clerk/nextjs/server';

export default clerkMiddleware();

export const config = {
  matcher: [
    '/((?!_next|[^?]*\\.(?:html?|css|js(?!on)|jpe?g|webp|png|gif|svg|ttf|woff2?|ico|csv|docx?|xlsx?|zip|webmanifest)).*)',
    '/(api|trpc)(.*)',
  ],
};
```

**Important:** This uses `clerkMiddleware()`, **NOT** the deprecated `authMiddleware()`.

### 2. Root Layout (`src/app/layout.tsx`)

Wraps the entire app with `<ClerkProvider>`:

```typescript
import {
  ClerkProvider,
  SignInButton,
  SignUpButton,
  SignedIn,
  SignedOut,
  UserButton,
} from "@clerk/nextjs";

export default function RootLayout({ children }) {
  return (
    <ClerkProvider>
      <html lang="en">
        <body>
          <header>
            <SignedOut>
              <SignInButton mode="modal">
                <button>Sign In</button>
              </SignInButton>
              <SignUpButton mode="modal">
                <button>Sign Up</button>
              </SignUpButton>
            </SignedOut>
            <SignedIn>
              <UserButton afterSignOutUrl="/" />
            </SignedIn>
          </header>
          {children}
        </body>
      </html>
    </ClerkProvider>
  );
}
```

### 3. Protected Dashboard (`src/app/dashboard/page.tsx`)

Uses `auth()` and `currentUser()` from `@clerk/nextjs/server`:

```typescript
import { auth, currentUser } from '@clerk/nextjs/server';
import { redirect } from 'next/navigation';

export default async function DashboardPage() {
  const { userId } = await auth();
  
  if (!userId) {
    redirect('/sign-in');
  }

  const user = await currentUser();
  // Render dashboard...
}
```

**Important:** These are **async** functions in App Router.

### 4. Sign-In/Sign-Up Pages

Using catch-all routes `[[...sign-in]]` and `[[...sign-up]]`:

```typescript
import { SignIn } from '@clerk/nextjs';

export default function Page() {
  return (
    <div className="flex min-h-screen items-center justify-center bg-gray-50">
      <SignIn />
    </div>
  );
}
```

## 🔐 Authentication Flow

1. **Unauthenticated User:**
   - Sees "Sign In" and "Sign Up" buttons in header
   - Landing page shows "Get Started Free" CTA
   - Accessing `/dashboard` redirects to `/sign-in`

2. **Sign Up:**
   - User clicks "Sign Up" → Modal opens (or redirect to `/sign-up`)
   - Clerk handles email verification
   - User is redirected to home page or dashboard

3. **Sign In:**
   - User clicks "Sign In" → Modal opens (or redirect to `/sign-in`)
   - Clerk authenticates
   - User is redirected to dashboard

4. **Authenticated User:**
   - Sees `<UserButton>` in header (profile picture)
   - Landing page shows "Go to Dashboard" CTA
   - Can access `/dashboard` and other protected routes

## 🎨 UI Components Used

### Clerk Components

- `<ClerkProvider>` - Root provider
- `<SignInButton>` - Trigger sign-in flow
- `<SignUpButton>` - Trigger sign-up flow
- `<UserButton>` - User profile dropdown
- `<SignedIn>` - Show content only to authenticated users
- `<SignedOut>` - Show content only to unauthenticated users
- `<SignIn>` - Full sign-in form component
- `<SignUp>` - Full sign-up form component

### Server Helpers

- `auth()` - Get auth state (userId, sessionId, etc.)
- `currentUser()` - Get full user object

All imported from `@clerk/nextjs` or `@clerk/nextjs/server`.

## 🚀 Next Steps

### Integrate with IORA MCP Server

Connect the dashboard to the IORA MCP backend:

```typescript
// In dashboard/page.tsx
const response = await fetch('http://localhost:7070/user/profile', {
  headers: {
    'Authorization': `Bearer ${await auth().getToken()}`
  }
});
```

### Add API Key Management UI

Create a component to call the IORA MCP `/user/api-keys` endpoints:

```typescript
// Create API key
const createKey = async (name: string) => {
  const token = await auth().getToken();
  const res = await fetch('http://localhost:7070/user/api-keys', {
    method: 'POST',
    headers: {
      'Authorization': `Bearer ${token}`,
      'Content-Type': 'application/json'
    },
    body: JSON.stringify({ name, permissions: ['tools:read', 'tools:write'] })
  });
  return res.json();
};
```

### Add Organization Features

Enable organizations in Clerk Dashboard, then add:

```typescript
import { OrganizationSwitcher } from '@clerk/nextjs';

// In layout or dashboard
<OrganizationSwitcher />
```

## ⚠️ Important Notes

### What NOT to Do (Outdated Patterns)

❌ **DO NOT** use `authMiddleware()` (deprecated)
❌ **DO NOT** use `_app.tsx` (Pages Router - outdated)
❌ **DO NOT** import from `@clerk/nextjs/app-beta` (old beta)
❌ **DO NOT** use `withAuth()` wrapper (deprecated)

### What to DO (Current Best Practices)

✅ **DO** use `clerkMiddleware()` from `@clerk/nextjs/server`
✅ **DO** use App Router (`app/` directory)
✅ **DO** use `async/await` with `auth()` and `currentUser()`
✅ **DO** wrap app with `<ClerkProvider>` in `app/layout.tsx`
✅ **DO** store keys only in `.env.local`

## 📚 Resources

- [Clerk Next.js Quickstart](https://clerk.com/docs/quickstarts/nextjs) ✅ Official
- [Clerk App Router Guide](https://clerk.com/docs/nextjs/guides/development/custom-sign-in-or-up-page) ✅ Official
- [Clerk Components Reference](https://clerk.com/docs/components/overview)
- [Clerk Backend SDK](https://clerk.com/docs/references/nextjs/overview)

---

**Last Updated:** October 3, 2025  
**Clerk SDK Version:** `@clerk/nextjs@latest`  
**Next.js Version:** App Router (13+)  
**Status:** ✅ Fully Implemented

