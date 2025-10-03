import { auth, currentUser } from '@clerk/nextjs/server';
import { redirect } from 'next/navigation';

export default async function DashboardPage() {
  const { userId } = await auth();
  
  if (!userId) {
    redirect('/sign-in');
  }

  const user = await currentUser();

  return (
    <div className="min-h-screen bg-gray-50 p-8">
      <div className="max-w-4xl mx-auto">
        <h1 className="text-3xl font-bold text-gray-900 mb-6">
          Welcome to IORA Dashboard
        </h1>
        
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
              <span className="font-medium">User ID:</span> {userId}
            </p>
          </div>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          <div className="bg-white rounded-lg shadow p-6">
            <h3 className="text-lg font-semibold text-gray-800 mb-3">API Keys</h3>
            <p className="text-gray-600 mb-4">
              Manage your IORA API keys for programmatic access.
            </p>
            <button className="bg-[#6c47ff] text-white rounded-lg px-4 py-2 text-sm hover:bg-[#5a3ad1] transition-colors">
              Create API Key
            </button>
          </div>

          <div className="bg-white rounded-lg shadow p-6">
            <h3 className="text-lg font-semibold text-gray-800 mb-3">Usage Statistics</h3>
            <p className="text-gray-600 mb-4">
              View your API usage and billing information.
            </p>
            <div className="space-y-2">
              <div className="flex justify-between">
                <span className="text-gray-600">Tier:</span>
                <span className="font-medium text-gray-900">Free</span>
              </div>
              <div className="flex justify-between">
                <span className="text-gray-600">Requests this month:</span>
                <span className="font-medium text-gray-900">0 / 10,000</span>
              </div>
            </div>
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
            <h3 className="text-lg font-semibold text-gray-800 mb-3">Upgrade Plan</h3>
            <p className="text-gray-600 mb-4">
              Unlock more features with Pro or Enterprise tiers.
            </p>
            <button className="bg-gradient-to-r from-[#6c47ff] to-[#5a3ad1] text-white rounded-lg px-4 py-2 text-sm hover:opacity-90 transition-opacity">
              Upgrade to Pro
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}

