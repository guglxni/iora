'use client';

import { useEffect, useState } from 'react';
import { User } from '@clerk/nextjs/server';

interface ApiKey {
  id: string;
  name: string;
  keyPrefix: string;
  createdAt: string;
  lastUsedAt?: string;
  expiresAt?: string;
  permissions: string[];
}

interface UsageStats {
  tier: string;
  limits: {
    requestsPerMinute: number;
    requestsPerMonth: number;
  };
  usage: {
    requestsThisMonth: number;
    requestsToday: number;
    lastRequest?: string;
  };
  remaining: {
    requestsThisMonth: number;
  };
}

interface DashboardClientProps {
  user: User | null;
}

export default function DashboardClient({ user }: DashboardClientProps) {
  const [apiKeys, setApiKeys] = useState<ApiKey[]>([]);
  const [usage, setUsage] = useState<UsageStats | null>(null);
  const [loading, setLoading] = useState(true);
  const [creatingKey, setCreatingKey] = useState(false);
  const [newKeyName, setNewKeyName] = useState('');
  const [createdKey, setCreatedKey] = useState<{ key: string; id: string } | null>(null);

  useEffect(() => {
    fetchData();
  }, []);

  const fetchData = async () => {
    try {
      setLoading(true);

      // Fetch API keys and usage in parallel
      const [apiKeysResponse, usageResponse] = await Promise.all([
        fetch('/api/user/api-keys'),
        fetch('/api/user/usage')
      ]);

      if (apiKeysResponse.ok) {
        const apiKeysData = await apiKeysResponse.json();
        setApiKeys(apiKeysData.data || []);
      }

      if (usageResponse.ok) {
        const usageData = await usageResponse.json();
        setUsage(usageData.data || null);
      }
    } catch (error) {
      console.error('Error fetching data:', error);
    } finally {
      setLoading(false);
    }
  };

  const createApiKey = async () => {
    if (!newKeyName.trim()) return;

    try {
      setCreatingKey(true);
      const response = await fetch('/api/user/api-keys', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          name: newKeyName,
          permissions: ['tools:read', 'tools:write'],
          expiresInDays: 90
        }),
      });

      if (response.ok) {
        const data = await response.json();
        setCreatedKey({ key: data.data.key, id: data.data.id });
        setNewKeyName('');
        // Refresh API keys list
        fetchData();
      }
    } catch (error) {
      console.error('Error creating API key:', error);
    } finally {
      setCreatingKey(false);
    }
  };

  const copyToClipboard = (text: string) => {
    navigator.clipboard.writeText(text);
  };

  if (loading) {
    return (
      <div className="min-h-screen bg-gray-50 p-8">
        <div className="max-w-4xl mx-auto">
          <div className="animate-pulse">
            <div className="h-8 bg-gray-200 rounded w-1/3 mb-6"></div>
            <div className="h-32 bg-gray-200 rounded mb-6"></div>
            <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
              <div className="h-48 bg-gray-200 rounded"></div>
              <div className="h-48 bg-gray-200 rounded"></div>
            </div>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-gray-50 p-8">
      <div className="max-w-4xl mx-auto">
        <h1 className="text-3xl font-bold text-gray-900 mb-6">
          Welcome to IORA Dashboard
        </h1>

        {/* User Profile */}
        <div className="bg-white rounded-lg shadow p-6 mb-6">
          <h2 className="text-xl font-semibold text-gray-800 mb-4">User Profile</h2>
          <div className="space-y-2">
            <p className="text-gray-600">
              <span className="font-medium">Name:</span> {user?.firstName} {user?.lastName}
            </p>
            <p className="text-gray-600">
              <span className="font-medium">Email:</span> {user?.emailAddresses[0]?.emailAddress}
            </p>
            <p className="text-gray-600">
              <span className="font-medium">User ID:</span> <code className="bg-gray-100 px-2 py-1 rounded text-sm">{user?.id}</code>
            </p>
          </div>
        </div>

        {/* API Keys Management */}
        <div className="bg-white rounded-lg shadow p-6 mb-6">
          <h2 className="text-xl font-semibold text-gray-800 mb-4">API Keys</h2>

          {/* Create New Key */}
          {!createdKey && (
            <div className="mb-6 p-4 border border-gray-200 rounded-lg">
              <h3 className="font-medium text-gray-800 mb-2">Create New API Key</h3>
              <div className="flex gap-3">
                <input
                  type="text"
                  value={newKeyName}
                  onChange={(e) => setNewKeyName(e.target.value)}
                  placeholder="Enter key name..."
                  className="flex-1 px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-[#6c47ff] focus:border-transparent"
                />
                <button
                  onClick={createApiKey}
                  disabled={creatingKey || !newKeyName.trim()}
                  className="bg-[#6c47ff] text-white px-4 py-2 rounded-md hover:bg-[#5a3ad1] disabled:opacity-50 disabled:cursor-not-allowed"
                >
                  {creatingKey ? 'Creating...' : 'Create Key'}
                </button>
              </div>
            </div>
          )}

          {/* Created Key Display */}
          {createdKey && (
            <div className="mb-6 p-4 bg-green-50 border border-green-200 rounded-lg">
              <h3 className="font-medium text-green-800 mb-2">✅ API Key Created Successfully!</h3>
              <div className="bg-green-100 p-3 rounded border font-mono text-sm break-all">
                {createdKey.key}
              </div>
              <div className="mt-3 flex gap-2">
                <button
                  onClick={() => copyToClipboard(createdKey.key)}
                  className="bg-green-600 text-white px-3 py-1 rounded text-sm hover:bg-green-700"
                >
                  Copy to Clipboard
                </button>
                <button
                  onClick={() => setCreatedKey(null)}
                  className="bg-gray-500 text-white px-3 py-1 rounded text-sm hover:bg-gray-600"
                >
                  Done
                </button>
              </div>
              <p className="text-sm text-green-700 mt-2">
                ⚠️ Save this key securely. It will not be shown again.
              </p>
            </div>
          )}

          {/* API Keys List */}
          <div className="space-y-3">
            {apiKeys.length === 0 ? (
              <p className="text-gray-500 text-center py-4">No API keys created yet.</p>
            ) : (
              apiKeys.map((key) => (
                <div key={key.id} className="flex items-center justify-between p-3 border border-gray-200 rounded-lg">
                  <div>
                    <h4 className="font-medium text-gray-800">{key.name}</h4>
                    <p className="text-sm text-gray-600 font-mono">{key.keyPrefix}</p>
                    <p className="text-xs text-gray-500">
                      Created: {new Date(key.createdAt).toLocaleDateString()}
                      {key.lastUsedAt && ` • Last used: ${new Date(key.lastUsedAt).toLocaleDateString()}`}
                    </p>
                  </div>
                  <button className="text-red-600 hover:text-red-800 text-sm">
                    Revoke
                  </button>
                </div>
              ))
            )}
          </div>
        </div>

        {/* Usage Statistics */}
        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          <div className="bg-white rounded-lg shadow p-6">
            <h3 className="text-lg font-semibold text-gray-800 mb-3">Usage Statistics</h3>
            {usage ? (
              <div className="space-y-3">
                <div className="flex justify-between">
                  <span className="text-gray-600">Current Tier:</span>
                  <span className={`font-medium ${usage.tier === 'free' ? 'text-green-600' : usage.tier === 'pro' ? 'text-blue-600' : 'text-purple-600'}`}>
                    {usage.tier.charAt(0).toUpperCase() + usage.tier.slice(1)}
                  </span>
                </div>
                <div className="flex justify-between">
                  <span className="text-gray-600">Requests this month:</span>
                  <span className="font-medium">
                    {usage.usage.requestsThisMonth.toLocaleString()} / {usage.limits.requestsPerMonth.toLocaleString()}
                  </span>
                </div>
                <div className="flex justify-between">
                  <span className="text-gray-600">Requests today:</span>
                  <span className="font-medium">{usage.usage.requestsToday.toLocaleString()}</span>
                </div>
                {usage.usage.lastRequest && (
                  <div className="flex justify-between">
                    <span className="text-gray-600">Last request:</span>
                    <span className="font-medium text-sm">{new Date(usage.usage.lastRequest).toLocaleString()}</span>
                  </div>
                )}
              </div>
            ) : (
              <p className="text-gray-500">Loading usage data...</p>
            )}
          </div>

          <div className="bg-white rounded-lg shadow p-6">
            <h3 className="text-lg font-semibold text-gray-800 mb-3">Organizations</h3>
            <p className="text-gray-600 mb-4">
              Manage your team and organization settings.
            </p>
            <button className="border border-gray-300 text-gray-700 rounded-lg px-4 py-2 text-sm hover:bg-gray-50 transition-colors">
              View Organizations
            </button>
          </div>

          <div className="bg-white rounded-lg shadow p-6">
            <h3 className="text-lg font-semibold text-gray-800 mb-3">Quick Actions</h3>
            <div className="space-y-2">
              <a
                href="/demo"
                className="block w-full text-center bg-blue-600 text-white rounded-lg px-4 py-2 text-sm hover:bg-blue-700 transition-colors"
              >
                Try Interactive Demo
              </a>
              <button className="w-full text-center border border-gray-300 text-gray-700 rounded-lg px-4 py-2 text-sm hover:bg-gray-50 transition-colors">
                View API Documentation
              </button>
            </div>
          </div>

          <div className="bg-white rounded-lg shadow p-6">
            <h3 className="text-lg font-semibold text-gray-800 mb-3">Upgrade Plan</h3>
            <p className="text-gray-600 mb-4">
              Unlock more features with Pro or Enterprise tiers.
            </p>
            <button className="bg-gradient-to-r from-[#6c47ff] to-[#5a3ad1] text-white rounded-lg px-4 py-2 text-sm hover:opacity-90 transition-opacity w-full">
              Upgrade to Pro
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}

